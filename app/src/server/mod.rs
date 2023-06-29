//!
//! Server
//!

use crate::{
    cmd::{Get, Ping, Set},
    models::{Connection, PersonRepository},
    protobuffer::{self, EchoRequest, EchoResponse, KeyValueRequest, KeyValueResponse},
};
use colored::*;
use derive_builder::*;
use std::{io::ErrorKind, pin::Pin, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};
use tracing::{error, info, instrument};

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn std::error::Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

/// Simply Echo Server
#[cfg_attr(feature = "server", derive(Debug, Builder))]
#[builder(pattern = "owned")]
pub struct EchoServer {
    person: PersonRepository,
    connection: Connection,
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send>>;
type EchoResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl protobuffer::echo_server::Echo for EchoServer {
    type ServerStreamingEchoStream = ResponseStream;
    type BidirectionalStreamingEchoStream = ResponseStream;

    async fn get_value(&self, req: Request<KeyValueRequest>) -> EchoResult<KeyValueResponse> {
        let key_value_request = req.into_inner();
        let key = key_value_request.key;
        let cmd = Get::new(key);

        match cmd.apply(&self.person, &self.connection).await {
            Ok(value) => Ok(Response::new(KeyValueResponse {
                status: value,
                error: None,
            })),
            Err(err) => Ok(Response::new(KeyValueResponse {
                status: "Error".to_owned(),
                error: Some(format!("{:?}", err)),
            })),
        }
    }

    #[instrument(skip(self, req))]
    async fn set_value(&self, req: Request<KeyValueRequest>) -> EchoResult<KeyValueResponse> {
        info!(message = "set_value".blue().to_string());
        info!(message = format!("{:?}", req.remote_addr().unwrap()));

        let key_value_request = req.into_inner();
        let key = key_value_request.key;
        let value = key_value_request.value.unwrap();
        let cmd = Set::new(key, value);

        match cmd.apply(&self.person, &self.connection).await {
            Ok(_) => Ok(Response::new(KeyValueResponse {
                status: "Ok".to_owned(),
                error: None,
            })),
            Err(err) => Ok(Response::new(KeyValueResponse {
                status: "Error".to_owned(),
                error: Some(format!("{:?}", err)),
            })),
        }
    }

    #[instrument(skip(self, req))]
    async fn unary_echo(&self, req: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        info!(message = "unary_echo".blue().to_string());
        info!(message = format!("{:?}", req.remote_addr().unwrap()));

        let message = req.into_inner().message;
        let cmd = Ping::new(message);

        match cmd.apply(&self.connection).await {
            Ok(message) => Ok(Response::new(EchoResponse { message })),
            Err(_) => Ok(Response::new(EchoResponse {
                message: "nil".to_owned(),
            })),
        }
    }

    // NOTE:
    // #[instrument] append EchoServer in the tracing log stdout
    #[instrument]
    async fn client_streaming_echo(
        &self,
        req: Request<Streaming<EchoRequest>>,
    ) -> EchoResult<EchoResponse> {
        info!(messsage = "client_streaming_echo".blue().to_string());
        info!(message = format!("{:?}", req.remote_addr().unwrap()));

        let mut in_stream = req.into_inner();
        let (tx, mut rx) = mpsc::channel(128);

        tokio::spawn(async move {
            let mut result = Vec::new();
            while let Some(item) = in_stream.next().await {
                match item {
                    Ok(v) => {
                        result.push(v.message.to_owned());
                    }
                    Err(err) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == ErrorKind::BrokenPipe {
                                // here you can handle special case when client
                                // disconnected in unexpected way
                                error!("{}", "client disconnected: broken pipe".red());
                                break;
                            }
                        }
                        match tx.send(Err(err)).await {
                            Ok(_) => (),
                            Err(_err) => break, // response was droped
                        }
                    }
                }
            }
            tx.send(Ok(EchoResponse {
                message: result.join(""),
            }))
            .await
            .expect("working rx error");
            info!("{}", "stream ended".green());
        });

        if let Some(res) = rx.recv().await {
            match res {
                Ok(echo_response) => Ok(Response::new(echo_response)),
                Err(_) => Err(Status::unknown("unknown response")),
            }
        } else {
            Err(Status::unknown("unknown response"))
        }
    }

    #[instrument]
    async fn server_streaming_echo(
        &self,
        req: Request<EchoRequest>,
    ) -> EchoResult<Self::ServerStreamingEchoStream> {
        info!(messsage = "server_streaming_echo".blue().to_string());
        info!(message = format!("{:?}", req.remote_addr().unwrap()));

        // TODO: It should change to other implementation of streamed response
        // creating infinite stream with requested message
        let repeat = std::iter::repeat(EchoResponse {
            message: req.into_inner().message,
        });

        // TODO: What should be right throttle strategy?
        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

        // spawn and channel are required if you want handle "disconnect" functionality
        // the `out_stream` will not be polled after client disconnect
        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            // stream.next() happens when client stream.take() is invoked
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        // item (server response) was queued to be send to client
                        // TODO: it may implement total count of streamed items, for progress reporting
                    }
                    // TODO: Err is thrown when all streamed items are taken in client stream
                    // Not knowing if this is correct implementation
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
            }
            info!("{}", "\tclient disconnected".red());
        });

        let output_stream = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(output_stream) as Self::ServerStreamingEchoStream
        ))
    }

    #[instrument]
    async fn bidirectional_streaming_echo(
        &self,
        req: Request<Streaming<EchoRequest>>,
    ) -> EchoResult<Self::BidirectionalStreamingEchoStream> {
        info!(message = "bidirectional_streaming_echo".blue().to_string());
        info!(message = format!("{:?}", req.remote_addr().unwrap()));

        let mut in_stream = req.into_inner();
        let (tx, rx) = mpsc::channel(128);

        // this spawn here is required if you want to handle connection error.
        // If we just map `in_stream` and write it back as `out_stream` the `out_stream`
        // will be droped when connection error occurs and error will never be propagated
        // to mapped version of `in_stream`.
        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => tx
                        .send(Ok(EchoResponse { message: v.message }))
                        .await
                        .expect("working rx"),
                    Err(err) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == ErrorKind::BrokenPipe {
                                // here you can handle special case when client
                                // disconnected in unexpected way
                                error!("{}", "client disconnected: broken pipe".red());
                                break;
                            }
                        }
                        match tx.send(Err(err)).await {
                            Ok(_) => (),
                            Err(_err) => break, // response was droped
                        }
                    }
                }
            }
            info!("{}", "stream ended".green());
        });
        // echo just write the same data that was received
        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(out_stream) as Self::BidirectionalStreamingEchoStream
        ))
    }
}

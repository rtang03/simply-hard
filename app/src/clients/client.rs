use crate::{
    protobuffer::{echo_client::EchoClient, EchoRequest, KeyValueRequest},
    AppError,
};
use colored::*;
use std::time::Duration;
use tokio_stream::{Stream, StreamExt};
use tonic::{codegen::StdError, transport::Channel, Request};
use tracing::{error, info, instrument};

#[cfg_attr(feature = "cli", derive(Debug))]
pub struct Client {
    echo_client: EchoClient<Channel>,
}

impl Client {
    pub async fn connect<D>(addr: D) -> crate::Result<Client>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        match EchoClient::connect(addr).await {
            Ok(echo_client) => Ok(Client { echo_client }),
            Err(err) => Err(AppError::GrpcConnectError(err)),
        }
    }

    // infinite iterator of EchoRequests
    fn echo_requests_iter() -> impl Stream<Item = EchoRequest> {
        tokio_stream::iter(1..usize::MAX).map(|i| EchoRequest {
            message: format!("msg {:02}", i),
        })
    }

    #[instrument(skip(self, key))]
    pub async fn get_value(&mut self, key: String) {
        let request = Request::new(KeyValueRequest { key, value: None });
        info!(
            message = format!("{}", "Sending get_value request".blue()),
            key = %request.get_ref().key,
        );

        match self.echo_client.get_value(request).await {
            Ok(response) => {
                let message = match response.get_ref().error.clone() {
                    Some(err) => format!("\n{}", err.red()),
                    None => response.get_ref().status.clone(),
                };
                info!(
                    message = format!("{}", "Got a response".blue()),
                    response = %response.get_ref().status
                );
                println!("\n{message}");
            }
            Err(err) => error!(error = format!("{:?}", err)),
        }
    }

    #[instrument(skip(self, key, value))]
    pub async fn set_value(&mut self, key: String, value: String) {
        let request = Request::new(KeyValueRequest {
            key,
            value: Some(value),
        });
        info!(
            message = format!("{}", "Sending set_value request".blue()),
            key = %request.get_ref().key,
        );

        match self.echo_client.set_value(request).await {
            Ok(response) => {
                let message = match response.get_ref().error.clone() {
                    Some(err) => format!("\n{}", err.red()),
                    None => response.get_ref().status.clone(),
                };
                info!(
                    message = format!("{}", "Got a response".blue()),
                    response = %response.get_ref().status
                );
                println!("\n{message}");
            }
            Err(err) => error!(error = format!("{:?}", err)),
        }
    }

    #[instrument(skip(self))]
    pub async fn unary_echo(&mut self, message: String) {
        let request = Request::new(EchoRequest { message });
        info!(
            message = format!("{}", "Sending request".blue()),
            request = %request.get_ref().message
        );

        match self.echo_client.unary_echo(request).await {
            Ok(response) => {
                info!(
                    message = format!("{}", "Got a response".blue()),
                    response = %response.get_ref().message
                );
                println!("{}", response.get_ref().message)
            }
            Err(err) => error!(error = format!("{:?}", err)),
        }
    }

    #[instrument(skip(self))]
    pub async fn client_streaming_echo(&mut self, num: usize) {
        // input stream
        let in_stream = Self::echo_requests_iter().take(num);

        let response = self
            .echo_client
            .client_streaming_echo(in_stream)
            .await
            .unwrap();

        info!(
            message = format!("{}", "Got a response".blue()),
            response = %response.get_ref().message
        );
    }

    /// server side streaming - take num of element and disconnect
    #[instrument(skip(self))]
    pub async fn streaming_echo(&mut self, num: usize) {
        let request = Request::new(EchoRequest {
            message: "foo".into(),
        });

        info!(
            message = format!("{}", "Sending request".blue()),
            request = %request.get_ref().message
        );

        // TODO: change the message "foo"
        let stream = self
            .echo_client
            .server_streaming_echo(request)
            .await
            .unwrap()
            .into_inner();

        // stream is infinite - take just 5 elements and then disconnect
        // TODO: change it to a meaningful implementation
        let mut stream = stream.take(num);
        while let Some(item) = stream.next().await {
            println!("\treceived: {}", item.unwrap().message.blue());
        }
        // stream is droped here and the disconnect info is send to server
    }

    /// BidirectionalStreamingEcho is bidi streaming.
    #[instrument(skip(self))]
    pub async fn bidirectional_streaming_echo(&mut self, num: usize) {
        // input stream
        let in_stream = Self::echo_requests_iter().take(num);

        // output stream
        let response = self
            .echo_client
            .bidirectional_streaming_echo(in_stream)
            .await
            .unwrap();

        let mut resp_stream = response.into_inner();

        // TODO: change it a meaningful implementation
        while let Some(received) = resp_stream.next().await {
            let received = received.unwrap();
            println!("\treceived message: `{}`", received.message.blue());
        }
    }

    /// BidirectionalStreamingEcho is bidi streaming, with throttling
    #[instrument(skip(self))]
    pub async fn bidirectional_streaming_echo_throttle(&mut self, dur: Duration) {
        let in_stream = Self::echo_requests_iter().throttle(dur);

        let response = self
            .echo_client
            .bidirectional_streaming_echo(in_stream)
            .await
            .unwrap();

        let mut resp_stream = response.into_inner();

        while let Some(received) = resp_stream.next().await {
            let received = received.unwrap();
            println!("\treceived message: `{}`", received.message.blue());
        }
    }
}

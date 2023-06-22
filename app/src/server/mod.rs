// pub mod protobuffer {
//     include!("../echo.rs");
// }

use crate::protobuffer;
use crate::protobuffer::{EchoRequest, EchoResponse};
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tracing::{debug, info};

#[derive(Debug, Default)]
pub struct EchoServer {}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send>>;
type EchoResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl protobuffer::echo_server::Echo for EchoServer {
    type ServerStreamingEchoStream = ResponseStream;
    type BidirectionalStreamingEchoStream = ResponseStream;

    #[tracing::instrument]
    async fn unary_echo(&self, _: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        info!("received request");
        debug!("sending response");
        Err(Status::unimplemented("not implemented"))
    }

    #[tracing::instrument]
    async fn client_streaming_echo(
        &self,
        _: Request<Streaming<EchoRequest>>,
    ) -> EchoResult<EchoResponse> {
        info!("received request");
        Err(Status::unimplemented("not implemented"))
    }

    #[tracing::instrument]
    async fn server_streaming_echo(
        &self,
        _req: Request<EchoRequest>,
    ) -> EchoResult<Self::ServerStreamingEchoStream> {
        info!("received request");
        Err(Status::unimplemented("not implemented"))
    }

    #[tracing::instrument]
    async fn bidirectional_streaming_echo(
        &self,
        _req: Request<Streaming<EchoRequest>>,
    ) -> EchoResult<Self::BidirectionalStreamingEchoStream> {
        info!("received request");
        Err(Status::unimplemented("not implemented"))
    }
}

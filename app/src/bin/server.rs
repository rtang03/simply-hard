// DEVELOPMENT NOTE:
// https://www.youtube.com/watch?v=JkSa-qA2jnY&t=106s
// https://github.com/hyperium/tonic/blob/master/examples/src/streaming/server.rs
//
// cargo build --release --bin simply-server
// cargo run --bin simply-server
// ./simply-server --port 50051

use app::DEFAULT_PORT;
use clap::Parser;
use futures::Stream;
use gupload::gupload_service_server::{GuploadService, GuploadServiceServer};
use gupload::{
    Chunk, FileRequest, FileResponse, HealthCheckRequest, HealthCheckResponse, UploadStatus,
};
use std::net::ToSocketAddrs;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod gupload {
    include!("../gupload.rs");
}

#[derive(Debug, Default)]
pub struct GuploadServiceImpl {}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<FileResponse, Status>> + Send>>;

#[tonic::async_trait]
impl GuploadService for GuploadServiceImpl {
    type DownloadStream = ResponseStream;

    async fn upload(
        &self,
        request: Request<Streaming<Chunk>>,
    ) -> Result<Response<UploadStatus>, Status> {
        let number_of_teams: i32 = 3;
        info!(
            number_of_teams,
            "We've got {} upload request!", number_of_teams
        );
        Ok(Response::new(UploadStatus {
            message: "OK".to_owned(),
            code: 0,
        }))
    }

    async fn download(
        &self,
        request: Request<FileRequest>,
    ) -> Result<Response<Self::DownloadStream>, Status> {
        let (tx, rx) = mpsc::channel(128);
        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::DownloadStream
        ))
    }

    async fn check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        Ok(Response::new(HealthCheckResponse {
            status: 0,
            received_at: "some timestamp".to_owned(),
        }))
    }
}

#[derive(Parser, Debug)]
#[clap(name = "simply-server", version, author, about = "Simply server")]
struct Cli {
    #[clap(long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> app::Result<()> {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFAULT_PORT);
    let gupload = GuploadServiceImpl::default();

    Server::builder()
        .add_service(GuploadServiceServer::new(gupload))
        .serve(
            format!("[::1]:{}", port)
                .to_socket_addrs()
                .unwrap()
                .next()
                .unwrap(),
        )
        .await
        .unwrap();

    Ok(())
}

// DEVELOPMENT NOTE:
// https://www.youtube.com/watch?v=JkSa-qA2jnY&t=106s
// https://github.com/hyperium/tonic/blob/master/examples/src/streaming/server.rs
// https://tokio.rs/tokio/topics/shutdown
//
// cargo build --release --bin simply-server
// cargo run --bin simply-server
// ./simply-server --port 50051

use app::{protobuffer, server::EchoServer, DEFAULT_PORT};
use clap::Parser;
use std::net::ToSocketAddrs;
use tonic::transport::Server;
use tracing::{info, info_span, Level};
use tracing_subscriber::FmtSubscriber;

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
        .with_max_level(Level::DEBUG)
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFAULT_PORT);
    let server = EchoServer::default();

    info!(message = "Starting server:", port);

    let graceful_shutdown = async {
        if let Ok(result) = tokio::signal::ctrl_c().await {
            // TODO: add logic
            info!(message = "Shutting down");
            result
        }
    };

    let server = Server::builder()
        .trace_fn(|_| info_span!("echo_server"))
        .add_service(protobuffer::echo_server::EchoServer::new(server))
        .serve_with_shutdown(
            format!("[::1]:{}", port)
                .to_socket_addrs()
                .unwrap()
                .next()
                .unwrap(),
            graceful_shutdown,
        );

    info!(message = "Server listening on", ?port);

    server.await?;

    Ok(())
}

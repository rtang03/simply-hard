// DEVELOPMENT NOTE:
// https://www.youtube.com/watch?v=JkSa-qA2jnY&t=106s
// https://github.com/hyperium/tonic/blob/master/examples/src/streaming/server.rs
// https://tokio.rs/tokio/topics/shutdown
//
// cargo build --release --bin simply-server
// cargo run --bin simply-server
// ./simply-server --port 50051
extern crate derive_builder;

use app::{
    protobuffer, server::EchoServerBuilder, Connection, PersonRepository, Settings, DEFAULT_PORT,
    GLOBAL_SETTINGS,
};
use clap::Parser;
use colored::*;

#[derive(Parser, Debug)]
#[clap(name = "simply-server", version, author, about = "Simply server")]
struct Cli {
    #[clap(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> Result<(), tonic::transport::Error> {
    use tonic::transport::Server;
    use tracing::{info, info_span, Level};
    use tracing_subscriber::FmtSubscriber;

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

    Settings::new();
    Settings::print_config("Initial").await;

    let cli = Cli::parse();

    let person_repository = PersonRepository::default();

    let simply_server = EchoServerBuilder::default()
        .person(person_repository)
        .connection(Connection::new().await)
        .build()
        .unwrap();

    let graceful_shutdown = async {
        if let Ok(result) = tokio::signal::ctrl_c().await {
            // TODO: add logic
            info!("{}", "gracefully shutting down".green());
            result
        }
    };
    let addr = format!("[::1]:{}", cli.port).parse().unwrap();

    info!("{}", format!("Server listening on {:?}", addr).blue());

    let server = Server::builder()
        .trace_fn(|_| info_span!("echo_server"))
        .add_service(protobuffer::echo_server::EchoServer::new(simply_server))
        .serve_with_shutdown(addr, graceful_shutdown);

    tokio::spawn(async {
        if let Err(err) = GLOBAL_SETTINGS.watch().await {
            println!("watch error: {:?}", err);
            println!("Quitting...");
        }
    });

    server.await?;

    Ok(())
}

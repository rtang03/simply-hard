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
    models::PersonRepository, protobuffer, server::EchoServerBuilder, Connection, InMemoryDatabase,
    Settings, DEFAULT_PORT, GLOBAL_SETTINGS,
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
async fn main() -> app::Result<()> {
    use tonic::transport::Server;
    use tracing::{info, info_span};

    app::server::set_up_logging()?;

    Settings::new();
    Settings::print_config("Initial").await;

    let cli = Cli::parse();

    let person_repository = PersonRepository::default();

    let simply_server = EchoServerBuilder::default()
        .person(person_repository)
        .connection(<InMemoryDatabase as Connection>::new().await)
        .build()
        .unwrap();

    let graceful_shutdown = async {
        if let Ok(result) = tokio::signal::ctrl_c().await {
            // TODO: add logic
            info!("{}", "gracefully shutting down".green());
            result
        }
    };
    let addr = format!("0.0.0.0:{}", cli.port).parse().unwrap();
    // let addr = format!("[::1]:{}", cli.port).parse().unwrap();

    info!("{}", format!("Server listening on {:?}", addr).blue());

    let server = Server::builder()
        .trace_fn(|_| info_span!("bootstrap_echo_server"))
        .add_service(protobuffer::echo_server::EchoServer::new(simply_server))
        .serve_with_shutdown(addr, graceful_shutdown);

    tokio::spawn(async {
        if let Err(err) = GLOBAL_SETTINGS.watch().await {
            println!("watch error: {:?}", err);
            println!("Quitting...");
        }
    });

    match server.await {
        Ok(_) => {
            app::server::shutdown_tracer_provider();
            Ok(())
        }
        Err(err) => Err(app::AppError::TonicError(err)),
    }
}

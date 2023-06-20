// DEVELOPMENT NOTE:
// https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html
// https://docs.rs/clap/latest/clap/_tutorial/index.html
//
// cargo build --release --bin simply-cli
// cargo run --bin simply-cli
//

use app::DEFAULT_PORT;
use clap::{Parser, Subcommand};
use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Parser, Debug)]
#[clap(
    name = "simply-cli",
    version,
    author,
    about = "Perform simple task in hard way"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[clap(name = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[clap(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Ping server
    #[command(arg_required_else_help = true)]
    Ping { msg: String },
}

/// Entry point for CLI tool.
///
/// The `[tokio::main]` annotation signals that the Tokio runtime should be
/// started when the function is called. The body of the function is executed
/// within the newly spawned runtime.
///
/// `flavor = "current_thread"` is used here to avoid spawning background
/// threads. The CLI tool use case benefits more by being lighter instead of
/// multi-threaded.
#[tokio::main(flavor = "current_thread")]
async fn main() -> app::Result<()> {
    // Enable logging
    tracing_subscriber::fmt::try_init()?;

    // Parse command line arguments
    // let cli = Cli::parse();

    // match cli.command {
    //     Command::Ping { msg } => {
    //         println!("echo {:?}", msg);
    //     }
    // }

    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(BtcPaymentRequest {
        from_add: "123456".to_owned(),
        to_add: "789".to_owned(),
        amount: 22,
    });

    let response = client.send_payment(request).await?;

    println!("Response: {:?}", response);

    Ok(())
}

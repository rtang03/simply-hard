// DEVELOPMENT NOTE:
// https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html
// https://docs.rs/clap/latest/clap/_tutorial/index.html
//
// cargo build --release --bin simply-cli
// cargo run --bin simply-cli
//

use app::{clients::streaming_echo, protobuffer::echo_client::EchoClient, DEFAULT_PORT};
use clap::{Parser, Subcommand};
use tracing::info;

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

    // #[clap(name = "hostname", long, default_value = "127.0.0.1")]
    // host: String,
    #[clap(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Ping server
    #[command(arg_required_else_help = true)]
    StreamEcho { num: usize },
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
    let cli = Cli::parse();

    // Get the remote address to connect to
    let addr = format!("http://[::1]:{}", cli.port);

    info!(message = "Connecting to", addr);

    let mut client = EchoClient::connect(addr).await?;

    match cli.command {
        Command::StreamEcho { num } => {
            println!("Repeat {} time(s)", num);
            streaming_echo(&mut client, num).await;
        }
    }

    Ok(())
}

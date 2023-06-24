// DEVELOPMENT NOTE:
// https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html
// https://docs.rs/clap/latest/clap/_tutorial/index.html
//
// cargo build --release --bin simply-cli
// cargo run --bin simply-cli
// ./simply-cli stream-echo 5

use app::{clients::Client, DEFAULT_PORT};
use clap::{Parser, Subcommand};
use colored::*;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

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

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Subcommand)]
enum Command {
    /// server side stream, e.g. stream-echo 5
    #[command(arg_required_else_help = true)]
    StreamEcho { num: usize },

    /// bidirection stream, e.g. bidi-stream-echo 5
    #[command(arg_required_else_help = true)]
    BidiStreamEcho { num: usize },

    /// client side stream, e.g. client-stream-echo 5
    #[command(arg_required_else_help = true)]
    ClientStreamEcho { num: usize },

    /// unary echo, e.g. unary-echo
    #[command(arg_required_else_help = false)]
    UnaryEcho,
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
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .compact()
        .with_file(false)
        .with_line_number(true)
        .with_thread_ids(false)
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Parse command line arguments
    let cli = Cli::parse();

    // Get the remote address to connect to
    let addr = format!("http://[::1]:{}", cli.port);

    info!(message = format!("{}", "Connecting".blue()), addr);

    let mut client = match Client::connect(addr).await {
        Ok(client) => client,
        Err(_) => panic!("{}", "failed to establish connection".red()),
    };

    match cli.command {
        Command::StreamEcho { num } => {
            client.streaming_echo(num).await;
        }
        Command::BidiStreamEcho { num } => {
            client.bidirectional_streaming_echo(num).await;
        }
        Command::UnaryEcho => {
            client.unary_echo().await;
        }
        Command::ClientStreamEcho { num } => {
            client.client_streaming_echo(num).await;
        }
    }

    Ok(())
}

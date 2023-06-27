// NOTE:
// https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html
// https://docs.rs/clap/latest/clap/_tutorial/index.html
// https://github.com/hyperium/tonic/blob/master/examples/src/mock/mock.rs
//
// cargo build --release --bin simply-cli
// cargo run --bin simply-cli
// ./simply-cli stream-echo 5

use app::{clients::Client, DEFAULT_PORT};
use clap::{Parser, Subcommand};

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
    #[command(arg_required_else_help = true)]
    UnaryEcho { message: String },
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
#[cfg(feature = "cli")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> app::Result<()> {
    use colored::*;
    use tracing::{info, Level};
    use tracing_subscriber::FmtSubscriber;

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
        Command::UnaryEcho { message } => {
            client.unary_echo(message).await;
        }
        Command::ClientStreamEcho { num } => {
            client.client_streaming_echo(num).await;
        }
    }

    Ok(())
}

// NOTE:
// may replace #[test] by #[tokio::test(flavor = "current_thread")]
// It will give a short code, by removing tokio::runtime::Builder::new_multi_thread()
// Shorter syntax is tradeoff by... the dim button "Run Test" in VS Code will disappear
// This (longer) code remains here, for self-learning purposes
//
// Alternatively, can use this one, if single thread is desired
// #[tokio::test(flavor = "current_thread")]

/// test cli to issue unary_echo command
#[test]
fn test_cli_unary_echo() {
    use app::{
        protobuffer::{self, echo_client::EchoClient},
        server::EchoServerBuilder,
    };
    use tonic::{
        transport::{Endpoint, Server, Uri},
        Request, Response,
    };
    use tower::service_fn;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let (client, server) = tokio::io::duplex(1024);
            let simply_server = EchoServerBuilder::default()
                // .connection(Connection::new().await)
                .build()
                .unwrap();

            tokio::spawn(async move {
                Server::builder()
                    .add_service(protobuffer::echo_server::EchoServer::new(simply_server))
                    .serve_with_incoming(tokio_stream::iter(vec![Ok::<_, std::io::Error>(server)]))
                    .await
            });

            // Move client to an option so we can _move_ the inner value
            // on the first attempt to connect. All other attempts will fail.
            let mut client = Some(client);
            let channel = Endpoint::try_from("http://[::]:50051")
                .unwrap()
                .connect_with_connector(service_fn(move |_: Uri| {
                    let client = client.take();

                    async move {
                        if let Some(client) = client {
                            Ok(client)
                        } else {
                            Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "Client already taken",
                            ))
                        }
                    }
                }))
                .await
                .unwrap();

            let mut client = EchoClient::new(channel);

            let request = Request::new(protobuffer::EchoRequest {
                message: "foo".to_owned(),
            });

            let response: Response<protobuffer::EchoResponse> =
                client.unary_echo(request).await.unwrap();

            assert_eq!(response.get_ref().message, "foo");
        })
}

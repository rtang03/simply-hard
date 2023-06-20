// DEVELOPMENT NOTE:
// https://www.youtube.com/watch?v=JkSa-qA2jnY&t=106s
//
// cargo build --release --bin simply-server
// cargo run --bin simply-server
//

use app::DEFAULT_PORT;
use clap::Parser;
use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentRequest, BtcPaymentResponse};
use tonic::{transport::Server, Request, Response, Status};
use tracing::Level;
// use tracing::info;
use tracing_subscriber::FmtSubscriber;

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct BitcoinService {}

#[tonic::async_trait]
impl Bitcoin for BitcoinService {
    async fn send_payment(
        &self,
        request: Request<BtcPaymentRequest>,
    ) -> Result<Response<BtcPaymentResponse>, Status> {
        println!("Got a payment request: {:?}", request);

        let req = request.into_inner();

        let reply = BtcPaymentResponse {
            successful: true,
            message: format!("Sent {}Btc to {}.", req.amount, req.to_add),
        };

        Ok(Response::new(reply))
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
    // let number_of_teams: i32 = 3;
    // info!(number_of_teams, "We've got {} teams!", number_of_teams);

    // let cli = Cli::parse();
    // let port = cli.port.unwrap_or(DEFAULT_PORT);

    let addr = "[::1]:50051".parse()?;
    let btc_service = BitcoinService::default();

    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}

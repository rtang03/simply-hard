use crate::protobuffer::{echo_client::EchoClient, EchoRequest};
use colored::*;
use std::time::Duration;
use tokio_stream::{Stream, StreamExt};
use tonic::{codegen::StdError, transport::Channel, Request};
use tracing::instrument;

pub struct Client {
    echo_client: EchoClient<Channel>,
}

impl Client {
    pub async fn connect<D>(addr: D) -> crate::Result<Client>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let echo_client = EchoClient::connect(addr).await?;

        Ok(Client { echo_client })
    }

    // infinite iterator of EchoRequests
    fn echo_requests_iter() -> impl Stream<Item = EchoRequest> {
        tokio_stream::iter(1..usize::MAX).map(|i| EchoRequest {
            message: format!("msg {:02}", i),
        })
    }

    #[instrument(skip(self))]
    pub async fn unary_echo(&mut self) {
        let request = Request::new(EchoRequest {
            message: "foo".into(),
        });
        let response = self.echo_client.unary_echo(request).await.unwrap();
        println!("\t{:?}", response);
    }

    #[instrument(skip(self))]
    pub async fn client_streaming_echo(&mut self, num: usize) {
        // input stream
        let in_stream = Self::echo_requests_iter().take(num);

        let response = self
            .echo_client
            .client_streaming_echo(in_stream)
            .await
            .unwrap();

        println!("\t{:?}", response);
    }

    // TODO: what is (skip(self)) means?
    /// server side streaming - take num of element and disconnect
    #[instrument(skip(self))]
    pub async fn streaming_echo(&mut self, num: usize) {
        // TODO: change the message "foo"
        let stream = self
            .echo_client
            .server_streaming_echo(EchoRequest {
                message: "foo".into(),
            })
            .await
            .unwrap()
            .into_inner();

        // stream is infinite - take just 5 elements and then disconnect
        // TODO: change it to a meaningful implementation
        let mut stream = stream.take(num);
        while let Some(item) = stream.next().await {
            println!("\treceived: {}", item.unwrap().message.blue());
        }
        // stream is droped here and the disconnect info is send to server
    }

    /// BidirectionalStreamingEcho is bidi streaming.
    pub async fn bidirectional_streaming_echo(&mut self, num: usize) {
        // input stream
        let in_stream = Self::echo_requests_iter().take(num);

        // output stream
        let response = self
            .echo_client
            .bidirectional_streaming_echo(in_stream)
            .await
            .unwrap();

        let mut resp_stream = response.into_inner();

        // TODO: change it a meaningful implementation
        while let Some(received) = resp_stream.next().await {
            let received = received.unwrap();
            println!("\treceived message: `{}`", received.message.blue());
        }
    }

    pub async fn bidirectional_streaming_echo_throttle(&mut self, dur: Duration) {
        let in_stream = Self::echo_requests_iter().throttle(dur);

        let response = self
            .echo_client
            .bidirectional_streaming_echo(in_stream)
            .await
            .unwrap();

        let mut resp_stream = response.into_inner();

        while let Some(received) = resp_stream.next().await {
            let received = received.unwrap();
            println!("\treceived message: `{}`", received.message.blue());
        }
    }
}

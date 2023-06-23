use crate::protobuffer::{echo_client::EchoClient, EchoRequest};
use std::time::Duration;
use tokio_stream::{Stream, StreamExt};
use tonic::transport::Channel;

// infinite iterator of EchoRequests
fn echo_requests_iter() -> impl Stream<Item = EchoRequest> {
    tokio_stream::iter(1..usize::MAX).map(|i| EchoRequest {
        message: format!("msg {:02}", i),
    })
}

/// server side streaming - take num of element and disconnect
pub async fn streaming_echo(client: &mut EchoClient<Channel>, num: usize) {
    let stream = client
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
        println!("\treceived: {}", item.unwrap().message);
    }
    // stream is droped here and the disconnect info is send to server
}

/// BidirectionalStreamingEcho is bidi streaming.
pub async fn bidirectional_streaming_echo(client: &mut EchoClient<Channel>, num: usize) {
    // input stream
    let in_stream = echo_requests_iter().take(num);

    // output stream
    let response = client
        .bidirectional_streaming_echo(in_stream)
        .await
        .unwrap();

    let mut resp_stream = response.into_inner();

    // TODO: change it a meaningful implementation
    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        println!("\treceived message: `{}`", received.message);
    }
}

pub async fn bidirectional_streaming_echo_throttle(
    client: &mut EchoClient<Channel>,
    dur: Duration,
) {
    let in_stream = echo_requests_iter().throttle(dur);

    let response = client
        .bidirectional_streaming_echo(in_stream)
        .await
        .unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        println!("\treceived message: `{}`", received.message);
    }
}

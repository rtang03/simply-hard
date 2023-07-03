mod common;
use common::setup;

extern crate app;
// use app::{protobuffer, server::EchoServer};
// use tonic::transport::Server;

#[cfg(not(test))]
fn compare(a: i32, b: i32) -> bool {
    a + 10 > b
}

#[cfg(test)]
fn compare(a: i32, b: i32) -> bool {
    a > b
}

#[test]
fn some_int_test() {
    setup();
    println!("some_int_test");
    assert!(compare(10, 1));
}

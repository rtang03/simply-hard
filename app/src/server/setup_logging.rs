// NOTE
// https://github.com/rlebran/Rust-distributed-tracing-example/blob/master/common/src/lib.rs
// https://medium.com/better-programming/distributed-tracing-in-rust-b8eb2af3aff4
// https://github.com/open-telemetry/opentelemetry-rust/blob/main/examples/tracing-grpc/src/server.rs
// https://github.com/rthomas/rust-tonic-jaeger-example
// docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest

#[cfg(not(feature = "otel"))]
pub fn set_up_logging() -> crate::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .finish();

    // use that subscriber to process traces emitted after this point
    match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => Ok(()),
        Err(err) => Err(crate::AppError::TracingSetGlobalDefaultError(err)),
    }
}

#[cfg(feature = "otel")]
pub fn set_up_logging() -> crate::Result<()> {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    // use tonic as grpc layer here.
    // If you want to use grpcio. enable `grpc-sys` feature and use with_grpcio function here.
    // https://crates.io/crates/opentelemetry-otlp
    // https://docs.rs/opentelemetry-jaeger/0.18.0/opentelemetry_jaeger/
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("simply-server")
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Unable to initialize OtlpPipeline");

    // Create a tracing layer with the configured opentelemetry
    // https://crates.io/crates/tracing-opentelemetry
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // NOTES:
    // Parse an `EnvFilter` configuration from the `RUST_LOG`
    // RUST_LOG="DEBUG" cargo run --bin simply-server
    // FIXME: uncomment below for official release
    // let filter = tracing_subscriber::EnvFilter::from_default_env();
    let simply_filter = tracing_subscriber::EnvFilter::new("INFO");

    match tracing_subscriber::registry()
        .with(opentelemetry)
        .with(simply_filter)
        .with(fmt::Layer::default())
        .try_init()
    {
        Ok(_) => Ok(()),
        Err(err) => Err(crate::AppError::TracingSubscriberInitError(err)),
    }
}

#[cfg(not(feature = "otel"))]
pub fn shutdown_tracer_provider() {}

#[cfg(feature = "otel")]
pub fn shutdown_tracer_provider() {
    opentelemetry::global::shutdown_tracer_provider();
}

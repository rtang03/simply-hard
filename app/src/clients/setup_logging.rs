// NOTE:
// https://github.com/open-telemetry/opentelemetry-rust/blob/main/examples/tracing-grpc/src/client.rs

#[cfg(not(feature = "otel"))]
pub fn set_up_logging() -> crate::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .with_file(false)
        .with_line_number(true)
        .with_thread_ids(false)
        .finish();

    match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => Ok(()),
        Err(err) => Err(crate::AppError::TracingSetGlobalDefaultError(err)),
    }
}

#[cfg(feature = "otel")]
pub fn set_up_logging() -> crate::Result<()> {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("simply-cli")
        .install_simple()
        .expect("Unable to initialize OtlpPipeline");

    match tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
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

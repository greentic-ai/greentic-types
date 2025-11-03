//! Telemetry helpers exposed by `greentic-types`.

mod span_context;

pub use span_context::SpanContext;

#[cfg(feature = "telemetry-autoinit")]
pub use greentic_telemetry::init::TelemetryError;
#[cfg(feature = "telemetry-autoinit")]
pub use greentic_telemetry::{
    OtlpConfig, TelemetryCtx, init_otlp, layer_from_task_local, set_current_tenant_ctx,
};
#[cfg(feature = "telemetry-autoinit")]
pub use greentic_types_macros::main;
#[cfg(feature = "telemetry-autoinit")]
#[doc(hidden)]
pub use tokio::main as __tokio_main;

#[cfg(feature = "telemetry-autoinit")]
/// Installs the default Greentic telemetry stack using OTLP + task-local context injection.
pub fn install_telemetry(service_name: &str) -> Result<(), TelemetryError> {
    use alloc::boxed::Box;

    let endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:4317".into());
    let layer: Box<
        dyn tracing_subscriber::layer::Layer<tracing_subscriber::Registry> + Send + Sync + 'static,
    > = Box::new(layer_from_task_local());

    init_otlp(
        OtlpConfig {
            endpoint,
            service_name: service_name.into(),
            insecure: true,
        },
        vec![layer],
    )
}

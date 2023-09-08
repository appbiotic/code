use tracing_subscriber::{prelude::*, EnvFilter};

/// Initializes telemetry with default settings.
///
/// Filtering is done with the `RUST_LOG` environment variable.
pub fn init_telemetry() {
    // TODO: Add required variations, e.g., JSON
    let fmt = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(false)
        .compact();

    tracing_subscriber::registry()
        .with(fmt)
        .with(EnvFilter::from_default_env())
        .init();
}

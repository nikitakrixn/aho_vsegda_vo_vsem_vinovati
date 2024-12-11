use std::io::stdout;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::fmt::writer::MakeWriterExt;

pub fn init_logging(app_name: &str) {
    let file_appender = RollingFileAppender::builder()
        .filename_suffix("app.log")
        .rotation(Rotation::DAILY)
        .build("./logs")
        .expect("Could not create logging folder.");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,backend=debug,tower_http=debug,axum::rejection=trace".into()))
        .with(fmt::layer().with_writer(stdout.and(non_blocking)))
        .init();

    tracing::info!(
        app_name = app_name,
        version = env!("CARGO_PKG_VERSION"),
        "Application started"
    );
}

#[macro_export]
macro_rules! log_info {
    ($msg:expr $(, $($arg:tt)+)?) => {
        tracing::info!($msg $(, $($arg)+)?);
    };
}

#[macro_export]
macro_rules! log_error {
    ($msg:expr $(, $($arg:tt)+)?) => {
        tracing::error!($msg $(, $($arg)+)?);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($msg:expr $(, $($arg:tt)+)?) => {
        tracing::warn!($msg $(, $($arg)+)?);
    };
}
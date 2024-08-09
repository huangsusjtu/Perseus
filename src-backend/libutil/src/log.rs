use time::UtcOffset;
use tracing_core::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_log() -> anyhow::Result<Box<dyn FnOnce()>> {
    let log_level = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let local_time = tracing_subscriber::fmt::time::OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::well_known::Rfc3339,
    );

    let (stdoutlog, _guard) = tracing_appender::non_blocking(std::io::stdout());

    let stdout = fmt::Layer::new()
        .with_writer(stdoutlog)
        .with_timer(local_time.clone())
        .with_ansi(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true);

    let subscriber =
        tracing_subscriber::registry().with(stdout).with(log_level);
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(Box::new(move || {
        drop(_guard);
    }))
}

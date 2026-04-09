use tracing_appender::non_blocking::WorkerGuard;

pub fn setup_tracing() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::never(".", "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(tracing::Level::DEBUG)
        .with_ansi(false)
        .with_env_filter(tracing_subscriber::EnvFilter::new("spotify_lyrics_tui=debug"))
        .init();

    guard
}
use slog::Drain;

pub(crate) fn root_logger(verbose: bool) -> slog::Logger {
  let decorator = slog_term::TermDecorator::new().build();
  let drain = slog_term::FullFormat::new(decorator).build().fuse();
  let level = if verbose { slog::Level::Debug } else { slog::Level::Info };

  let both = if std::path::Path::new("log").exists() {
    match std::fs::OpenOptions::new()
      .create(true)
      .append(true)
      .open("log/{{project-name}}.log")
    {
      Ok(file) => {
        let json = slog_json::Json::default(file).fuse();
        let fused = std::sync::Mutex::new(slog::Duplicate::new(drain, json)).fuse();
        slog_async::Async::new(slog::LevelFilter::new(fused, level).fuse()).build().fuse()
      }
      Err(e) => {
        println!("Unable to create file at [log/{{project-name}}.log]: {}", e);
        slog_async::Async::new(slog::LevelFilter::new(drain, level).fuse()).build().fuse()
      }
    }
  } else {
    slog_async::Async::new(slog::LevelFilter::new(drain, level).fuse()).build().fuse()
  };

  let mut root = slog::Logger::root(both, slog::o!("version" => env!("CARGO_PKG_VERSION")));
  if verbose {
    root = root.new(slog::o!("loc" => slog::FnValue(|record| format!("{}/{}:{}", record.module(), record.file(), record.line()))));
  }
  root
}

use crate::files::FileService;

use slog;
use std::sync::Arc;

/// Contains information about the running application
#[derive(Clone, Debug)]
pub struct AppConfig {
  task: String,
  address: String,
  port: u16,
  verbose: bool,
  files: Arc<FileService>,
  root_logger: slog::Logger
}

impl AppConfig {
  pub fn new(task: String, address: String, port: u16, cfg_dir: String, verbose: bool, root_logger: slog::Logger) -> AppConfig {
    let files = Arc::new(FileService::new(&cfg_dir, &root_logger));
    AppConfig {
      task,
      address,
      port,
      verbose,
      files: Arc::clone(&files),
      root_logger
    }
  }

  pub fn task(&self) -> &String {
    &self.task
  }

  pub fn address(&self) -> &String {
    &self.address
  }

  pub fn port(&self) -> u16 {
    self.port
  }

  pub fn verbose(&self) -> bool {
    self.verbose
  }

  pub fn files(&self) -> &FileService {
    &self.files
  }

  pub fn root_logger(&self) -> &slog::Logger {
    &self.root_logger
  }
}

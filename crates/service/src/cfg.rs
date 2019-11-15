use crate::cache::ConnectionCache;
use crate::files::FileService;

use slog;
use std::sync::{Arc, RwLock};

/// Contains information about the running application
#[derive(Clone, Debug)]
pub struct AppConfig {
  task: String,
  address: String,
  port: u16,
  verbose: bool,
  files: Arc<FileService>,
  connections: Arc<RwLock<ConnectionCache>>,
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
      connections: Arc::new(RwLock::new(ConnectionCache::new(&root_logger))),
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

  pub fn connections(&self) -> &RwLock<ConnectionCache> {
    &self.connections
  }

  pub fn root_logger(&self) -> &slog::Logger {
    &self.root_logger
  }
}

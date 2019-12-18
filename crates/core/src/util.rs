use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time_ms() -> u128 {
  let start = SystemTime::now();
  start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum NotificationLevel {
  Info,
  Success,
  Warn,
  Error
}

impl std::fmt::Display for NotificationLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Self::Info => "Info",
      Self::Success => "Success",
      Self::Warn => "Warn",
      Self::Error => "Error"
    };
    write!(f, "{}", s)
  }
}

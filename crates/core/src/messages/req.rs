use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Sent from client to server, this shared model is used for all server communication
#[derive(Debug, Serialize, Deserialize)]
pub enum RequestMessage {
  Ping { v: i64 },
  GetVersion
}

impl RequestMessage {
  pub fn from_json(s: &str) -> Result<RequestMessage> {
    serde_json::from_str(&s).with_context(|| format!("Can't decode JSON RequestMessage from [{}]", s))
  }

  pub fn to_json(&self) -> Result<String> {
    serde_json::to_string_pretty(&self).with_context(|| "Can't encode JSON RequestMessage")
  }

  pub fn from_binary(b: &[u8]) -> Result<RequestMessage> {
    bincode::deserialize(&b).with_context(|| "Can't decode binary RequestMessage".to_string())
  }

  pub fn to_binary(&self) -> Result<Vec<u8>> {
    bincode::serialize(&self).with_context(|| "Can't encode binary RequestMessage")
  }
}

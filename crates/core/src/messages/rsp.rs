use crate::util::NotificationLevel;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Sent from server to client, this shared model is used for all client communication
#[allow(variant_size_differences)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ResponseMessage {
  Connected {
    connection_id: Uuid,
    user_id: Uuid,
    u: Box<crate::profile::UserProfile>,
    b: bool
  },
  ServerError {
    reason: String,
    content: String
  },
  Pong {
    v: i64
  },
  Notification {
    level: NotificationLevel,
    content: String
  }
}

impl ResponseMessage {
  pub fn from_json(s: &str) -> Result<ResponseMessage> {
    serde_json::from_str(&s).with_context(|| "Can't decode json ResponseMessage")
  }

  pub fn to_json(&self) -> Result<String> {
    serde_json::to_string_pretty(&self).with_context(|| "Can't encode json ResponseMessage")
  }

  pub fn from_binary(b: &[u8]) -> Result<ResponseMessage> {
    bincode::deserialize(&b).with_context(|| "Can't decode binary ResponseMessage")
  }

  pub fn to_binary(&self) -> Result<Vec<u8>> {
    bincode::serialize(&self).with_context(|| "Can't encode binary ResponseMessage")
  }
}

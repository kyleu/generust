use {{crate_name}}_core::ResponseMessage;

use uuid::Uuid;
use std::collections::{HashMap, HashSet};

pub trait SendCallback: Sync + Send {
  fn send_message(&self, msg: ResponseMessage) -> ();
}

pub struct ConnectionCache {
  connections: HashMap<Uuid, Box<dyn SendCallback>>,
  channels: HashMap<String, HashSet<Uuid>>,
  log: slog::Logger
}

impl std::fmt::Debug for ConnectionCache {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "ConnectionCache [{}] connections, [{}] channels",
      &self.connections.len(),
      &self.channels.len()
    )
  }
}

impl ConnectionCache {
  pub fn new(log: &slog::Logger) -> ConnectionCache {
    let log = log.new(slog::o!("service" => "connection-cache"));
    slog::debug!(log, "Connection cache created");
    ConnectionCache {
      connections: HashMap::new(),
      channels: HashMap::new(),
      log
    }
  }

  pub fn conn_list(&self) -> Vec<Uuid> {
    let mut conns: Vec<Uuid> = self.connections.keys().copied().collect();
    conns.sort();
    conns
  }

  pub fn channel_list(&self) -> Vec<(String, Vec<Uuid>)> {
    let mut channels: Vec<(String, Vec<Uuid>)> = self
      .channels
      .iter()
      .map(|v| {
        let mut ids: Vec<Uuid> = v.1.iter().copied().collect();
        ids.sort();
        (v.0.clone(), ids)
      })
      .collect();
    channels.sort();
    channels
  }

  pub fn add<F>(&mut self, key: &str, id: Uuid, f: Box<dyn SendCallback>) {
    let _ = self.connections.insert(id, f);
    match self.channels.get_mut(key) {
      Some(current) => {
        slog::debug!(
          self.log,
          "Adding additional connection [{}] to [{}], which has [{}] existing connections",
          id,
          key,
          current.len()
        );
        let _ = current.insert(id);
      }
      None => {
        slog::debug!(self.log, "Adding first connection [{}] to [{}]", id, key);
        let set: HashSet<Uuid> = vec![id].into_iter().collect();
        let _ = self.channels.insert(key.into(), set);
      }
    }
  }

  pub fn remove(&mut self, key: &str, id: Uuid) {
    let _ = self.connections.remove(&id);
    match self.channels.get_mut(key) {
      Some(current) => {
        if current.contains(&id) {
          let _ = current.remove(&id);
          slog::debug!(
            self.log,
            "Removed connection [{}] from [{}], leaving [{}] other connections",
            id,
            key,
            current.len()
          )
        } else {
          slog::debug!(
            self.log,
            "Could not find connection [{}] for [{}] among [{}] other connections",
            id,
            key,
            current.len()
          )
        }
      }
      None => slog::debug!(
        self.log,
        "Attempt to remove connection [{}] from [{}], which has no connections",
        id,
        key
      )
    }
  }

  pub fn send_channel(&self, key: &str, msg: ResponseMessage) {
    match &mut self.channels.get(key) {
      Some(current) => {
        slog::debug!(
          self.log,
          "Sending message [{:?}] to [{}], which contains [{}] connections",
          msg,
          key,
          current.len()
        );
        let _: Vec<_> = current
          .iter()
          .map(|id| match self.connections.get(id) {
            Some(f) => f.send_message(msg.clone()),
            None => slog::warn!(self.log, "Unable to send message")
          })
          .collect();
      }
      None => ()
    }
  }

  pub fn send_session(&self, id: &Uuid, msg: ResponseMessage) {
    match &mut self.connections.get(id) {
      Some(f) => {
        slog::debug!(self.log, "Sending message [{:?}] to connection [{}]", msg, id);
        f.send_message(msg);
      }
      None => ()
    }
  }
}

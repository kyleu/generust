use {{crate_name}}_core::ResponseMessage;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub trait SendCallback: Sync + Send {
  fn send_message(&self, msg: ResponseMessage) -> ();
}

pub struct ConnectionCache {
  connections: Arc<RwLock<HashMap<Uuid, Box<dyn SendCallback>>>>,
  channels: Arc<RwLock<HashMap<String, HashSet<Uuid>>>>,
  log: slog::Logger
}

impl std::fmt::Debug for ConnectionCache {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "ConnectionCache [{}] connections, [{}] channels",
      &self.connections.read().unwrap().len(),
      &self.channels.read().unwrap().len()
    )
  }
}

impl ConnectionCache {
  pub fn new(log: &slog::Logger) -> ConnectionCache {
    let log = log.new(slog::o!("service" => "connection-cache"));
    slog::debug!(log, "Connection cache created");
    ConnectionCache {
      connections: Arc::new(RwLock::new(HashMap::new())),
      channels: Arc::new(RwLock::new(HashMap::new())),
      log
    }
  }

  pub fn conn_list(&self) -> Vec<Uuid> {
    let mut conns: Vec<Uuid> = self.connections.read().unwrap().keys().copied().collect();
    conns.sort();
    conns
  }

  pub fn channel_list(&self) -> Vec<(String, Vec<Uuid>)> {
    let mut channels: Vec<(String, Vec<Uuid>)> = self.channels.read().unwrap().iter().map(|v| {
      let mut ids: Vec<Uuid> = v.1.iter().copied().collect();
      ids.sort();
      (v.0.clone(), ids)
    }).collect();
    channels.sort();
    channels
  }

  pub fn add<F>(&self, key: &str, id: Uuid, f: Box<dyn SendCallback>) {
    let _ = self.connections.write().unwrap().insert(id, f);
    let mut chan = self.channels.write().unwrap();
    match chan.get_mut(key) {
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
        let _ = chan.insert(key.into(), set);
      }
    }
  }

  pub fn remove(&self, key: &str, id: Uuid) {
    let _ = self.connections.write().unwrap().remove(&id);
    let mut chan = self.channels.write().unwrap();
    match chan.get_mut(key) {
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

  pub fn send_connection(&self, id: &Uuid, msg: ResponseMessage) {
    match &mut self.connections.read().unwrap().get(id) {
      Some(f) => {
        slog::debug!(self.log, "Sending message [{:?}] to connection [{}]", msg, &id);
        f.send_message(msg);
      }
      None => slog::warn!(self.log, "Message send attempted for missing connection [{}]", &id)
    }
  }

  pub fn send_channel(&self, key: &str, msg: ResponseMessage) {
    self.send_channel_except(key, vec![], msg)
  }

  pub fn send_channel_except(&self, key: &str, exclude: Vec<&Uuid>, msg: ResponseMessage) {
    match &mut self.channels.read().unwrap().get(key) {
      Some(current) => {
        let size = current.len();
        let filtered: Vec<&Uuid> = current
          .iter()
          .filter(|c| {
            println!("{:?} / {} == {}", exclude, c, !exclude.contains(c));
            !exclude.contains(c)
          })
          .collect();
        slog::debug!(
          self.log,
          "Sending message [{:?}] to [{}], using [{} of {}] connections",
          msg,
          key,
          filtered.len(),
          size
        );
        let _: Vec<_> = filtered
          .iter()
          .map(|id| match self.connections.read().unwrap().get(id) {
            Some(f) => f.send_message(msg.clone()),
            None => slog::warn!(self.log, "Unable to send message")
          })
          .collect();
      }
      None => ()
    }
  }
}

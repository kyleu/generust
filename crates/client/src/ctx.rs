use crate::socket::ws::ClientSocket;

use anyhow::Result;
use std::rc::Rc;
use std::sync::RwLock;
use uuid::Uuid;
use web_sys::{Document, Window};
use {{crate_name}}_core::profile::UserProfile;
use {{crate_name}}_core::RequestMessage;

#[derive(Debug)]
pub(crate) struct ClientContext {
  window: Window,
  document: Document,
  socket: ClientSocket,
  connection_id: Option<Uuid>,
  user_id: Option<Uuid>,
  user_profile: UserProfile
}

impl ClientContext {
  pub(crate) fn new() -> Result<Rc<RwLock<Self>>> {
    let binary = true;
    let window = web_sys::window().ok_or_else(|| anyhow::anyhow!("Can't find [window]"))?;
    let document = window.document().ok_or_else(|| anyhow::anyhow!("Can't find [document]"))?;
    let loc = &document.location().ok_or_else(|| anyhow::anyhow!("Can't find [location]"))?;
    let url = loc.href().map_err(|_| anyhow::anyhow!("Can't find [href]"))?;
    let socket = ClientSocket::new(&url, binary)?;
    let user_profile = UserProfile::default();

    let rc = Rc::new(RwLock::new(Self {
      window,
      document,
      socket,
      connection_id: None,
      user_id: None,
      user_profile
    }));

    crate::socket::ws_events::wire_socket(&Rc::clone(&rc));
    crate::socket::ws_events::on_load(&Rc::clone(&rc).read().expect("Cannot lock ClientContext for read"))?;

    debug!("[{}] has started", {{crate_name}}_core::APPNAME);

    Ok(rc)
  }

  pub(crate) const fn document(&self) -> &Document {
    &self.document
  }

  pub(crate) const fn socket(&self) -> &ClientSocket {
    &self.socket
  }

  pub(crate) const fn _connection_id(&self) -> &Option<Uuid> {
    &self.connection_id
  }

  pub(crate) const fn _user_id(&self) -> &Option<Uuid> {
    &self.user_id
  }

  pub(crate) const fn user_profile(&self) -> &UserProfile {
    &self.user_profile
  }

  pub(crate) fn on_connected(&mut self, connection_id: Uuid, user_id: Uuid, user_profile: UserProfile, binary: bool) {
    self.socket.set_binary(binary);
    self.connection_id = Some(connection_id);
    self.user_id = Some(user_id);
    self.user_profile = user_profile;
  }

  pub(crate) fn on_open(&self) -> Result<()> {
    debug!("Open success for [{}]", self.user_profile().name());
    Ok(())
  }

  pub(crate) fn send(&self, rm: &RequestMessage) -> Result<()> {
    self.socket.send(rm);
    Ok(())
  }

  pub(crate) fn on_event(&mut self, t: &str, k: &str, v: &str) -> Result<()> {
    crate::event_handler::EventHandler::handle(self, t, k, v)
  }

  pub(crate) fn on_error(&self) -> Result<()> {
    warn!("Error for [{}]", self.user_profile().name());
    Ok(())
  }

  pub(crate) fn on_close(&self) -> Result<()> {
    debug!("Close for [{}]", self.user_profile().name());
    Ok(())
  }
}

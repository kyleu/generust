use crate::socket::ws::ClientSocket;
use {{crate_name}}_core::profile::UserProfile;
use {{crate_name}}_core::{Error, RequestMessage, Result};

use std::rc::Rc;
use std::sync::RwLock;
use web_sys::{Document, Window};

#[derive(Debug)]
pub(crate) struct ClientContext {
  window: Window,
  document: Document,
  socket: ClientSocket,
  profile: UserProfile
}

impl ClientContext {
  pub(crate) fn new() -> Result<Rc<RwLock<ClientContext>>> {
    let binary = true;
    let window = web_sys::window().ok_or_else(|| Error::from("Can't find [window]"))?;
    let document = window.document().ok_or_else(|| Error::from("Can't find [document]"))?;
    let loc = &document.location().ok_or_else(|| Error::from("Can't find [location]"))?;
    let url = loc.href().map_err(|_| Error::from("Can't find [href]"))?;
    let socket = ClientSocket::new(&url, binary)?;
    let profile = UserProfile::default();

    let rc = Rc::new(RwLock::new(ClientContext {
      window,
      document,
      socket,
      profile
    }));

    crate::socket::ws_events::wire_socket(Rc::clone(&rc));
    crate::socket::ws_events::on_load(&Rc::clone(&rc).read().unwrap())?;

    debug!("[{}] has started", {{crate_name}}_core::APPNAME);

    Ok(rc)
  }

  pub(crate) fn document(&self) -> &Document {
    &self.document
  }

  pub(crate) fn socket(&self) -> &ClientSocket {
    &self.socket
  }

  pub(crate) fn profile(&self) -> &UserProfile {
    &self.profile
  }

  pub(crate) fn on_hello(&mut self, binary: bool, profile: UserProfile) {
    self.socket.set_binary(binary);
    self.profile = profile;
  }

  pub(crate) fn on_open(&self) -> Result<()> {
    Ok(())
  }

  pub(crate) fn send(&self, rm: RequestMessage) {
    self.socket.send(rm);
  }

  pub(crate) fn on_event(&mut self, t: &str, k: &str, v: &str) -> Result<()> {
    crate::event_handler::EventHandler::handle(self, t, k, v)
  }

  pub(crate) fn on_error(&self) -> Result<()> {
    Ok(())
  }

  pub(crate) fn on_close(&self) -> Result<()> {
    Ok(())
  }
}

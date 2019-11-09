use {{crate_name}}_core::messages::req::RequestMessage;
use {{crate_name}}_core::{Error, Result};

use wasm_bindgen::prelude::{Closure, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

#[derive(Clone, Debug)]
pub(crate) struct ClientSocket {
  url: String,
  binary: bool,
  pub(crate) ws: WebSocket
}

impl ClientSocket {
  pub(crate) fn new(curr_url: &str, binary: bool) -> Result<ClientSocket> {
    let url = calc_url(curr_url);
    let ws = WebSocket::new(&url).map_err(|e| Error::from(format!("Error creating WebSocket: {:?}", e)))?;
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    Ok(ClientSocket { url, binary, ws })
  }

  pub(crate) fn set_callbacks(
    &self, on_open: Box<dyn FnMut(JsValue)>, on_message: Box<dyn FnMut(MessageEvent)>, on_error: Box<dyn FnMut(ErrorEvent)>,
    on_close: Box<dyn FnMut(JsValue)>
  )
  {
    let onopen_callback = Closure::wrap(on_open);
    self.ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    let onmessage_callback = Closure::wrap(on_message);
    self.ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(on_error);
    self.ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let onclose_callback = Closure::wrap(on_close);
    self.ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();
  }

  pub(crate) fn set_binary(&mut self, b: bool) {
    self.binary = b;
  }

  pub(crate) fn send(&self, msg: RequestMessage) {
    if self.binary {
      self.send_binary(msg);
    } else {
      self.send_json(msg);
    }
  }

  pub(crate) fn send_json(&self, req: RequestMessage) {
    match req.to_json() {
      Ok(j) => match &self.ws.send_with_str(&j) {
        Ok(_) => (),
        Err(err) => error!("Error sending json message: [{:?}]", err)
      },
      Err(e) => {
        error!("Cannot serialize json RequestMessage: {}", e);
      }
    };
  }

  pub(crate) fn send_binary(&self, msg: RequestMessage) {
    match msg.to_binary() {
      Ok(mut v) => {
        let v: &mut [u8] = &mut v[..];
        match &self.ws.send_with_u8_array(v) {
          Ok(_) => (),
          Err(err) => error!("Error sending binary message: [{:?}]", err)
        }
      }
      Err(e) => {
        error!("Cannot serialize RequestMessage to binary: {}", e);
      }
    };
  }
}

fn calc_url(src: &str) -> String {
  let protocol = if src.contains("s:") { "wss" } else { "ws" };
  let cleaned = src
    .trim_start_matches("http")
    .trim_start_matches("file")
    .trim_start_matches("ws")
    .trim_start_matches('s')
    .trim_start_matches("://")
    .trim_end_matches('/')
    .split('#')
    .collect::<Vec<&str>>()[0]
    .split('?')
    .collect::<Vec<&str>>()[0];
  format!("{}://{}/connect", protocol, cleaned)
}

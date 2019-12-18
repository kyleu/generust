use crate::ctx::ClientContext;

use anyhow::Result;
use std::rc::Rc;
use std::sync::RwLock;

pub(crate) fn wire_socket(rc: &Rc<RwLock<ClientContext>>) {
  let on_open = {
    let ctx = Rc::clone(rc);
    Box::new(move |_j| {
      match crate::socket::ws_handlers::on_open(&ctx) {
        Ok(_) => {}
        Err(e) => error!("Error encountered running [on_open]: {}", e)
      };
    })
  };
  let on_message = {
    let ctx = Rc::clone(rc);
    Box::new(move |m: web_sys::MessageEvent| {
      match crate::socket::ws_handlers::on_message(&ctx, &m) {
        Ok(_) => {}
        Err(e) => error!("Error encountered running [on_message]: {}", e)
      };
    })
  };
  let on_error = {
    let ctx = Rc::clone(rc);
    Box::new(move |e: web_sys::ErrorEvent| match crate::socket::ws_handlers::on_error(&ctx, &e) {
      Ok(_) => {}
      Err(e) => error!("Error encountered running [on_error]: {}", e)
    })
  };
  let on_close = {
    let ctx = Rc::clone(rc);
    Box::new(move |_x| match crate::socket::ws_handlers::on_close(&ctx) {
      Ok(_) => {}
      Err(e) => error!("Error encountered running [on_close]: {}", e)
    })
  };

  let c = rc.read().expect("Cannot lock ClientContext for read");
  c.socket().set_callbacks(on_open, on_message, on_error, on_close);
}

pub(crate) fn on_load(ctx: &ClientContext) -> Result<()> {
  ctx.replace_template("socket-status", maud::html!("Connected"))
}

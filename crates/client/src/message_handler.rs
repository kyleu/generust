use crate::ctx::ClientContext;

use {{crate_name}}_core::{ResponseMessage, Result};
use {{crate_name}}_core::profile::UserProfile;

use maud::html;
use std::sync::RwLock;

pub(crate) struct MessageHandler {}

impl MessageHandler {
  pub(crate) fn handle(ctx: &RwLock<ClientContext>, msg: ResponseMessage) -> Result<()> {
    debug!("Message received: {:?}", msg);
    match msg {
      ResponseMessage::Hello { session_id, u, b } => on_hello(ctx, session_id, &u, b)?,
      ResponseMessage::Pong { v } => on_pong(ctx, v)?,
      _ => warn!("Unhandled ResponseMessage [{:?}]", msg)
    };
    Ok(())
  }
}

fn on_hello(ctx: &RwLock<ClientContext>, session_id: uuid::Uuid, u: &UserProfile, b: bool) -> Result<()> {
  ctx.write().unwrap().on_hello(session_id, u.clone(), b);
  let c = ctx.read().unwrap();
  let _ = c.append_template(
    "socket-results",
    "div",
    html!((format!("Hello received, {} connection", if b { "binary" } else { "text" })))
  )?;
  info!("Hello received");
  Ok(())
}

fn on_pong(ctx: &RwLock<ClientContext>, v: i64) -> Result<()> {
  let now = js_sys::Date::now() as i64;
  let msg = format!("Pong: [{}ms] elapsed", now - v);
  let _ = ctx.read().unwrap().append_template("socket-results", "div", html!((msg)))?;
  info!("{}", msg);
  Ok(())
}

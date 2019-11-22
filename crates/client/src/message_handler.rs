use crate::ctx::ClientContext;

use anyhow::Result;
use maud::html;
use std::sync::RwLock;
use {{crate_name}}_core::profile::UserProfile;
use {{crate_name}}_core::util::NotificationLevel;
use {{crate_name}}_core::ResponseMessage;

pub(crate) struct MessageHandler {}

impl MessageHandler {
  pub(crate) fn handle(ctx: &RwLock<ClientContext>, msg: ResponseMessage) -> Result<()> {
    debug!("Message received: {:?}", msg);
    match msg {
      ResponseMessage::Connected { connection_id, u, b } => on_connected(ctx, connection_id, &u, b)?,
      ResponseMessage::Pong { v } => on_pong(ctx, v)?,
      ResponseMessage::Notification { level, content } => on_notification(level, content)?,

      // Custom Messages
      _ => warn!("Unhandled ResponseMessage [{:?}]", msg)
    };
    Ok(())
  }
}

fn on_connected(ctx: &RwLock<ClientContext>, connection_id: uuid::Uuid, u: &UserProfile, b: bool) -> Result<()> {
  ctx.write().unwrap().on_connected(connection_id, u.clone(), b);
  let c = ctx.read().unwrap();
  let _ = c.append_template(
    "socket-results",
    "div",
    html!((format!("Connect message received, {} connection", if b { "binary" } else { "text" })))
  )?;
  Ok(())
}

fn on_notification(level: NotificationLevel, content: String) -> Result<()> {
  crate::logging::notify(level, &content);
  Ok(())
}

fn on_pong(ctx: &RwLock<ClientContext>, v: i64) -> Result<()> {
  let now = js_sys::Date::now() as i64;
  let msg = format!("Pong: [{}ms] elapsed", now - v);
  let _ = ctx.read().unwrap().append_template("socket-results", "div", html!((msg)))?;
  info!("{}", msg);
  Ok(())
}

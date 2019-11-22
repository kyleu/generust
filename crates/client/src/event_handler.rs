use crate::ctx::ClientContext;

use anyhow::Result;
use {{crate_name}}_core::RequestMessage;

pub(crate) struct EventHandler {}

impl EventHandler {
  pub(crate) fn handle(ctx: &ClientContext, t: &str, k: &str, v: &str) -> Result<()> {
    match t {
      "send-ping" => ctx.send(RequestMessage::Ping {
        v: js_sys::Date::now() as i64
      }),
      _ => warn!("Unhandled event [{}] with [k:{}], [v:{}]", t, k, v)
    }
    Ok(())
  }
}

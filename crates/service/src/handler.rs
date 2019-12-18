use crate::RequestContext;

use anyhow::Result;
use uuid::Uuid;
use {{crate_name}}_core::{RequestMessage, ResponseMessage};

/// Core application logic, routing [RequestMessage]({{crate_name}}_core::RequestMessage)s and emitting [ResponseMessage]({{crate_name}}_core::ResponseMessage)s.
#[derive(Debug)]
pub struct MessageHandler {
  connection_id: Uuid,
  channel_id: String,
  ctx: RequestContext,
  log: slog::Logger
}

impl MessageHandler {
  pub fn new(connection_id: Uuid, channel_id: String, ctx: RequestContext) -> Self {
    let o = slog::o!("connection" => format!("{}", connection_id), "service" => "message_handler", "channel" => channel_id.clone());
    let log = ctx.log().new(o);
    Self {
      connection_id,
      channel_id,
      ctx,
      log
    }
  }

  pub const fn connection_id(&self) -> &Uuid {
    &self.connection_id
  }

  pub const fn channel_id(&self) -> &String {
    &self.channel_id
  }

  pub const fn ctx(&self) -> &RequestContext {
    &self.ctx
  }

  pub fn log(&self) -> &slog::Logger {
    &self.log
  }

  pub fn on_open(&self) -> Result<()> {
    self.send_to_self(ResponseMessage::Connected {
      connection_id: *self.connection_id(),
      user_id: *self.ctx().user_id(),
      u: Box::new((*self.ctx.user_profile()).clone()),
      b: !self.ctx.app().verbose()
    })
  }

  pub fn on_closed(&self) -> Result<()> {
    slog::debug!(self.log, "Closing connection for [{}:{}]", self.connection_id, self.channel_id);
    Ok(())
  }

  pub fn on_message(&self, msg: RequestMessage) -> Result<()> {
    match msg {
      RequestMessage::Ping { v } => self.send_to_self(ResponseMessage::Pong { v }),
      msg => {
        slog::warn!(self.log, "Unhandled RequestMessage [{:?}]", msg);
        Ok(())
      }
    }
  }

  fn send_to_self(&self, msg: ResponseMessage) -> Result<()> {
    self.ctx().app().connections().send_connection(self.connection_id(), msg);
    Ok(())
  }

  fn _send_to_channel(&self, msg: &ResponseMessage) -> Result<()> {
    self.ctx().app().connections().send_channel(self.channel_id(), msg);
    Ok(())
  }

  fn _send_to_channel_except_self(&self, msg: &ResponseMessage) -> Result<()> {
    self
      .ctx()
      .app()
      .connections()
      .send_channel_except(self.channel_id(), &[self.connection_id()], msg);
    Ok(())
  }
}

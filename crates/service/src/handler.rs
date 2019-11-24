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
  pub fn new(connection_id: Uuid, channel_id: String, ctx: RequestContext) -> MessageHandler {
    let log = ctx
      .log()
      .new(slog::o!("connection" => format!("{}", connection_id), "service" => "message_handler", "channel" => channel_id.clone()));
    MessageHandler { connection_id, channel_id, ctx, log }
  }

  pub fn connection_id(&self) -> &Uuid {
    &self.connection_id
  }

  pub fn channel_id(&self) -> &String {
    &self.channel_id
  }

  pub fn ctx(&self) -> &RequestContext {
    &self.ctx
  }

  pub fn on_open(&self) -> Result<Vec<ResponseMessage>> {
    Ok(vec![ResponseMessage::Connected {
      connection_id: *self.connection_id(),
      user_id: *self.ctx().user_id(),
      u: Box::new((*self.ctx.user_profile()).clone()),
      b: !self.ctx.app().verbose()
    }])
  }

  pub fn on_closed(&self) -> Vec<ResponseMessage> {
    Vec::new()
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

  pub fn log(&self) -> &slog::Logger {
    &self.log
  }

  fn send_to_self(&self, msg: ResponseMessage) -> Result<()> {
    self.ctx().app().send_connection(self.connection_id(), msg);
    Ok(())
  }

  fn _send_to_channel(&self, msg: ResponseMessage) -> Result<()> {
    self.ctx().app().send_channel(self.channel_id(), msg);
    Ok(())
  }

  fn _send_to_channel_except_self(&self, msg: ResponseMessage) -> Result<()> {
    self.ctx().app().send_channel_except(self.channel_id(), vec!(self.connection_id()), msg);
    Ok(())
  }
}

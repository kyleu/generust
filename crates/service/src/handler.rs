use crate::RequestContext;
use {{crate_name}}_core::{RequestMessage, ResponseMessage, Result};

/// Core application logic, routing [RequestMessage]({{crate_name}}_core::RequestMessage)s and emitting [ResponseMessage]({{crate_name}}_core::ResponseMessage)s.
#[derive(Debug)]
pub struct MessageHandler {
  ctx: RequestContext,
  log: slog::Logger
}

impl MessageHandler {
  pub fn new(ctx: RequestContext) -> MessageHandler {
    let log = ctx.log().new(slog::o!("service" => "message_handler"));
    MessageHandler { ctx, log }
  }

  pub fn on_open(&self) -> Result<Vec<ResponseMessage>> {
    Ok(vec![ResponseMessage::Hello {
      u: Box::new((*self.ctx.user_profile()).clone()),
      b: !self.ctx.app().verbose()
    }])
  }

  pub fn on_closed(&self) -> Vec<ResponseMessage> {
    Vec::new()
  }

  pub fn on_message(&self, msg: RequestMessage) -> Result<Vec<ResponseMessage>> {
    let mut ret = Vec::new();
    match msg {
      RequestMessage::Ping { v } => ret.push(ResponseMessage::Pong { v }),
      msg => slog::warn!(self.log, "Unhandled RequestMessage [{:?}]", msg)
    }
    Ok(ret)
  }

  pub fn on_error(&self) {}

  pub fn log(&self) -> &slog::Logger {
    &self.log
  }
}

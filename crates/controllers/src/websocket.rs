use {{crate_name}}_core::{RequestMessage, ResponseMessage, Result};
use {{crate_name}}_service::handler::MessageHandler;
use {{crate_name}}_service::AppConfig;

use actix::{Actor, StreamHandler};
use actix_session::Session;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

struct ServerSocket {
  binary: bool,
  handler: MessageHandler
}

impl Actor for ServerSocket {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, wsc: &mut Self::Context) {
    let msgs = match self.handler.on_open() {
      Ok(m) => m,
      Err(e) => {
        slog::error!(self.handler.log(), "Unable to process on_open: {}", e);
        vec!()
      }
    };
    for msg in msgs {
      match self.send(msg, wsc) {
        Ok(_) => (),
        Err(e) => slog::warn!(self.handler.log(), "Unable to send initial open messages: {}", e)
      }
    }
  }
}

impl ServerSocket {
  fn handler(&self) -> &MessageHandler {
    &self.handler
  }

  fn handle_text(&self, s: String, wsc: &mut ws::WebsocketContext<Self>) -> Result<()> {
    let req = RequestMessage::from_json(&s)?;
    self.handle_message(req, wsc)
  }

  fn handle_binary(&self, bytes: bytes::Bytes, wsc: &mut ws::WebsocketContext<Self>) -> Result<()> {
    let b: &[u8] = bytes.as_ref();
    let req = RequestMessage::from_binary(&b.to_vec())?;
    self.handle_message(req, wsc)
  }

  fn handle_message(&self, req: RequestMessage, wsc: &mut ws::WebsocketContext<Self>) -> Result<()> {
    for msg in self.handler.on_message(req)? {
      self.send(msg, wsc)?;
    }
    Ok(())
  }

  fn handle_error(&self, e: &{{crate_name}}_core::Error, wsc: &mut ws::WebsocketContext<Self>) {
    slog::warn!(&self.handler().log(), "Error handling message: {}", e);
    let msg = ResponseMessage::ServerError {
      reason: format!("{}", e),
      content: "Error handling message".into()
    };
    match self.send(msg, wsc) {
      Ok(_) => (),
      Err(e) => slog::warn!(&self.handler().log(), "Error sending server error message: {}", e)
    }
  }

  fn send(&self, rsp: ResponseMessage, wsc: &mut ws::WebsocketContext<Self>) -> Result<()> {
    if self.binary {
      wsc.binary(rsp.to_binary()?);
    } else {
      wsc.text(rsp.to_json()?);
    }
    Ok(())
  }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for ServerSocket {
  fn handle(&mut self, msg: ws::Message, wsc: &mut Self::Context) {
    match msg {
      ws::Message::Ping(msg) => wsc.pong(&msg),
      ws::Message::Text(text) => match &self.handle_text(text, wsc) {
        Ok(_) => (),
        Err(e) => self.handle_error(&e, wsc)
      },
      ws::Message::Binary(bin) => match self.handle_binary(bin, wsc) {
        Ok(_) => (),
        Err(e) => self.handle_error(&e, wsc)
      },
      _ => ()
    }
  }
}

/// Available at `/connect` (WebSocket handler)
pub fn connect(
  session: Session, cfg: web::Data<AppConfig>, req: HttpRequest, stream: web::Payload
) -> std::result::Result<HttpResponse, Error> {
  let ctx = crate::req_context(&session, &cfg, &req, "connect");
  let binary = match req.query_string() {
    x if x.contains("t=b") => true,
    x if x.contains("t=j") => false,
    _ => !cfg.verbose()
  };

  let handler = MessageHandler::new(ctx);
  let socket = ServerSocket { binary, handler };
  ws::start(socket, &req, stream)
}

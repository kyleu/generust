use crate::util::ctx::add_flash;

use actix_session::Session;
use actix_web::web::{Data, Form, Path};
use actix_web::{HttpRequest, HttpResponse};

use {{crate_name}}_core::ResponseMessage;
use {{crate_name}}_service::AppConfig;

/// Available at `/admin`
pub fn list(session: Session, cfg: Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {{crate_name}}_templates::admin::list(&ctx, router))
}

/// Available at `/admin/conn`
pub fn connections(session: Session, cfg: Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    let conn = ctx.app().connections().read().unwrap();
    {{crate_name}}_templates::admin::connections(&ctx, router, conn.conn_list(), conn.channel_list())
  })
}

#[derive(Debug, serde::Deserialize)]
pub struct SendForm {
  level: String,
  content: String
}

/// Available at `/admin/conn/{id}`
pub fn connection_detail(session: Session, cfg: Data<AppConfig>, id: Path<uuid::Uuid>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    {{crate_name}}_templates::admin::connection_detail(&ctx, router, *id)
  })
}

/// Available by posting to `/admin/conn/{id}`
pub fn connection_send(session: Session, cfg: Data<AppConfig>, id: Path<uuid::Uuid>, req: HttpRequest, f: Form<SendForm>) -> HttpResponse {
  crate::redir(&session, &cfg, req, |ctx, router| {
    let msg = ResponseMessage::Notification {
      level: f.level.clone(),
      content: f.content.clone()
    };
    let conn = ctx.app().connections().read().unwrap();
    slog::info!(
      ctx.log(),
      "Sent admin message [{}::{}] to connection [{}]",
      &f.level,
      &f.content,
      &id
    );
    conn.send_connection(&id, msg);
    add_flash(&session, ctx.log(), "success", &format!("Sent message to connection [{}]", &id));
    router.route_simple("admin.connections")
  })
}

/// Available at `/admin/channel/{id}`
pub fn channel_detail(session: Session, cfg: Data<AppConfig>, key: Path<String>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    {{crate_name}}_templates::admin::channel_detail(&ctx, router, &key)
  })
}

/// Available by posting to `/admin/channel/{id}`
pub fn channel_send(session: Session, cfg: Data<AppConfig>, key: Path<String>, req: HttpRequest, f: Form<SendForm>) -> HttpResponse {
  crate::redir(&session, &cfg, req, |ctx, router| {
    let msg = ResponseMessage::Notification {
      level: f.level.clone(),
      content: f.content.clone()
    };
    let conn = ctx.app().connections().read().unwrap();
    slog::info!(
      ctx.log(),
      "Sending admin message [{}::{}] to channel [{}]",
      &f.level,
      &f.content,
      &key
    );
    conn.send_channel(&key, msg);
    add_flash(&session, ctx.log(), "success", &format!("Sent message to channel [{}]", &key));
    router.route_simple("admin.connections")
  })
}

/// Available at `/admin/settings`
pub fn settings(session: Session, cfg: Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {{crate_name}}_templates::settings::settings(&ctx, router))
}

/// Available by posting to `/admin/settings`
pub fn settings_post(session: Session, cfg: Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {{crate_name}}_templates::settings::settings(&ctx, router))
}

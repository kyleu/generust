use {{crate_name}}_core::Result;
use {{crate_name}}_service::AppConfig;
use {{crate_name}}_service::RequestContext;

use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::web::{Data, Path};
use actix_web::{HttpRequest, HttpResponse};

pub(crate) fn act<F>(session: &Session, cfg: &Data<AppConfig>, req: &HttpRequest, f: F) -> HttpResponse
where F: Fn(&RequestContext) -> Result<maud::Markup> {
  let ctx = crate::req_context(&session, &cfg, &req, "index");
  rsp(&ctx, f(&ctx))
}

pub(crate) fn rsp(ctx: &RequestContext, res: Result<maud::PreEscaped<String>>) -> HttpResponse {
  match res {
    Ok(html) => ok(html.into_string()),
    Err(e) => {
      slog::warn!(ctx.log(), "{}", e);
      err(ctx, &e)
    }
  }
}

pub(crate) fn redir<F>(session: &Session, cfg: &Data<AppConfig>, req: &HttpRequest, f: F) -> HttpResponse
where F: Fn(&RequestContext) -> Result<String> {
  let ctx = crate::req_context(&session, &cfg, &req, "index");
  match f(&ctx) {
    Ok(path) => HttpResponse::SeeOther().header(LOCATION, path).finish().into_body(),
    Err(e) => {
      slog::warn!(ctx.log(), "{}", e);
      err(&ctx, &e)
    }
  }
}

pub(crate) fn ok(content: String) -> HttpResponse {
  HttpResponse::Ok().content_type("text/html; charset=utf-8").body(content)
}

pub(crate) fn err(ctx: &RequestContext, e: &{{crate_name}}_core::Error) -> HttpResponse {
  let content = match {{crate_name}}_templates::error::exception(ctx, e) {
    Ok(c) => c.into_string(),
    Err(e) => format!("A critical system error has occurred: {}", e.to_string())
  };
  HttpResponse::InternalServerError()
    .content_type("text/html; charset=utf-8")
    .body(content)
}

pub(crate) fn not_found(session: Session, cfg: Data<AppConfig>, path: Path<String>, req: HttpRequest) -> HttpResponse {
  let ctx = crate::req_context(&session, &cfg, &req, "index");
  let content = match {{crate_name}}_templates::error::not_found(&ctx, &path) {
    Ok(c) => c.into_string(),
    Err(e) => format!("A critical system error has occurred: {}", e.to_string())
  };
  HttpResponse::NotFound().content_type("text/html; charset=utf-8").body(content)
}

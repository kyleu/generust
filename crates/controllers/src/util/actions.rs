use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::web::{Data, Path};
use actix_web::{HttpRequest, HttpResponse};
use anyhow::Result;
use {{crate_name}}_service::{AppConfig, RequestContext};
use {{crate_name}}_service::Router;

pub(crate) fn act<F>(session: &Session, cfg: &Data<AppConfig>, req: HttpRequest, f: F) -> HttpResponse
where F: Fn(&RequestContext, &dyn Router) -> Result<maud::Markup> {
  let ctx = crate::req_context(session, cfg, "index");
  let router = crate::util::router::RequestRouter::new(req);
  rsp(&ctx, &router, f(&ctx, &router))
}

pub(crate) fn rsp(ctx: &RequestContext, router: &dyn Router, res: Result<maud::PreEscaped<String>>) -> HttpResponse {
  match res {
    Ok(html) => ok(html.into_string()),
    Err(e) => {
      slog::warn!(ctx.log(), "{}", e);
      err(ctx, router, &e)
    }
  }
}

pub(crate) fn redir<F>(session: &Session, cfg: &Data<AppConfig>, req: HttpRequest, f: F) -> HttpResponse
where F: Fn(&RequestContext, &dyn Router) -> Result<String> {
  let ctx = crate::req_context(session, cfg, "index");
  let router = crate::util::router::RequestRouter::new(req);
  match f(&ctx, &router) {
    Ok(path) => HttpResponse::SeeOther().header(LOCATION, path).finish().into_body(),
    Err(e) => {
      slog::warn!(ctx.log(), "{}", e);
      err(&ctx, &router, &e)
    }
  }
}

pub(crate) fn ok(content: String) -> HttpResponse {
  HttpResponse::Ok().content_type("text/html; charset=utf-8").body(content)
}

pub(crate) fn err(ctx: &RequestContext, router: &dyn Router, e: &anyhow::Error) -> HttpResponse {
  let content = match {{crate_name}}_templates::error::exception(ctx, router, e) {
    Ok(c) => c.into_string(),
    Err(e) => format!("A critical system error has occurred: {}", e.to_string())
  };
  HttpResponse::InternalServerError()
    .content_type("text/html; charset=utf-8")
    .body(content)
}

pub(crate) fn not_found(session: Session, cfg: Data<AppConfig>, path: Path<String>, req: HttpRequest) -> HttpResponse {
  let ctx = crate::req_context(&session, &cfg, "index");
  let router = crate::util::router::RequestRouter::new(req);
  let content = match {{crate_name}}_templates::error::not_found(&ctx, &router, &path) {
    Ok(c) => c.into_string(),
    Err(e) => format!("A critical system error has occurred: {}", e.to_string())
  };
  HttpResponse::NotFound().content_type("text/html; charset=utf-8").body(content)
}

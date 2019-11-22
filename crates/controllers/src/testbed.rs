use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

use {{crate_name}}_service::AppConfig;

/// Available at `/testbed/{key}`
pub fn testbed_key(session: Session, cfg: web::Data<AppConfig>, key: web::Path<String>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    let k: &str = &key;
    match k {
      "dump" => {{crate_name}}_templates::testbed::dump(&ctx, router),
      "gallery" => {{crate_name}}_templates::testbed::gallery(&ctx, router),
      "prototype" => {{crate_name}}_templates::testbed::prototype(&ctx, router),
      "scroll" => {{crate_name}}_templates::testbed::scroll(&ctx, router),
      _ => Err(anyhow::anyhow!(format!("Cannot find testbed matching [{}]", key)))
    }
  })
}

use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

use {{crate_name}}_core::Error;
use {{crate_name}}_service::AppConfig;

/// Available at `/testbed/{key}`
pub fn testbed_key(session: Session, cfg: web::Data<AppConfig>, key: web::Path<String>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, &req, |ctx| {
    let k: &str = &key;
    match k {
      "dump" => {{crate_name}}_templates::testbed::dump(&ctx),
      "gallery" => {{crate_name}}_templates::testbed::gallery(&ctx),
      "prototype" => {{crate_name}}_templates::testbed::prototype(&ctx),
      "scroll" => {{crate_name}}_templates::testbed::scroll(&ctx),
      _ => Err(Error::from(format!("Cannot find testbed matching [{}]", key)))
    }
  })
}

use {{crate_name}}_assets::Asset;

use actix_web::body::Body;
use actix_web::web;
use actix_web::HttpResponse;
use mime_guess::from_path;
use std::borrow::Cow;

/// Available at `/static/{path}`
pub fn static_file(path: web::Path<(String,)>) -> HttpResponse {
  get(&path.0)
}

/// Available at `/favicon.ico`
pub fn favicon() -> HttpResponse {
  get("favicon.ico")
}

fn get(name: &str) -> HttpResponse {
  match Asset::get(&name) {
    Some(content) => {
      let body: Body = match content {
        Cow::Borrowed(bytes) => bytes.into(),
        Cow::Owned(bytes) => bytes.into()
      };
      HttpResponse::Ok()
        .content_type(from_path(name).first_or_octet_stream().as_ref())
        .body(body)
    }
    None => HttpResponse::NotFound().body("404 Not Found")
  }
}

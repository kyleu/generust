use actix_web::HttpRequest;

use {{crate_name}}_service::ctx::Router;

use {{crate_name}}_core::{Error, Result};

pub(crate) struct RequestRouter {
  req: HttpRequest
}

impl RequestRouter {
  pub(crate) fn new(req: HttpRequest) -> RequestRouter {
    RequestRouter { req }
  }
}

impl Router for RequestRouter {
  fn route(&self, path: &str, args: &[&str]) -> Result<String> {
    self
      .req
      .url_for(path, args)
      .map(|x| x.path().into())
      .map_err(|_| Error::from(format!("Unable to find route for [{}]", path)))
  }
}

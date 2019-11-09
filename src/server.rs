use {{crate_name}}_core::{Error, Result};

use {{crate_name}}_controllers::routes::add_routes;
use {{crate_name}}_service::AppConfig;

use actix_service::Service;
use actix_session::CookieSession;
use actix_web::{App, HttpServer};
use futures::future::Future;
use std::time::SystemTime;

pub(crate) fn start_server(cfg: AppConfig, port_tx: std::sync::mpsc::Sender<u16>) -> Result<()> {
  let server = {
    let cfg = cfg.clone();
    HttpServer::new(move || {
      App::new()
        .data(cfg.clone())
        .wrap(
          CookieSession::signed(&[0; 32])
            .http_only(true)
            .name({{crate_name}}_core::APPNAME)
            .secure(false)
        )
        .wrap(actix_web::middleware::Compress::default())
        .wrap_fn(|req, srv| {
          let p = req.path().to_owned();
          let start_time = SystemTime::now();
          let cfg: AppConfig = match req.app_data::<AppConfig>() {
            Some(ad) => ad.get_ref().to_owned(),
            None => panic!("Missing AppConfig data reference!")
          };
          let useful = !p.starts_with("/static");
          if useful {
            slog::trace!(cfg.root_logger(), "Request received for path [{}]", p);
          }
          srv.call(req).map(move |res| {
            if useful {
              let elapsed_time = SystemTime::now()
                .duration_since(start_time)
                .ok()
                .map(|x| x.as_micros() as f64)
                .get_or_insert(0f64)
                .to_owned();
              let ms = elapsed_time / 1000.0;
              slog::debug!(
                cfg.root_logger(), "{}", res.response().status();
                "path" => &p, "status" => format!("{}", res.response().status().as_u16()), "elapsed" => format!("{}", ms)
              );
            }
            res
          })
        })
        .configure(|s| add_routes(s))
    })
  };

  match server.bind(format!("{}:{}", &cfg.address(), cfg.port())) {
    Ok(s) => {
      let port = s.addrs()[0].port();
      let _ = port_tx.send(port);
      let msg = format!("[{{project-name}}] started, open http://{}:{} to get going!", cfg.address(), port);
      slog::info!(cfg.root_logger(), "{}", msg);
      s.run().map_err(|e| Error::from(format!("Error creating web server: {:?}", e)))
    }
    Err(e) => {
      let msg = format!("Error starting server on port [{}]: {}", cfg.port(), e);
      slog::info!(cfg.root_logger(), "{}", msg);
      Err(Error::from(msg))
    }
  }
}

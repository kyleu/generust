use actix_session::Session;
use actix_web::web::Data;
use uuid::Uuid;

use {{crate_name}}_service::AppConfig;
use {{crate_name}}_service::RequestContext;

const FLASH_KEY: &str = "{{project-name}}-flash";
const SESSION_ID_KEY: &str = "{{project-name}}-session";

/// Creates a [RequestContext]({{crate_name}}_service::RequestContext) from an HTTP request
pub fn req_context(session: &Session, cfg: &Data<AppConfig>, action_key: &'static str) -> RequestContext {
  let app_cfg = cfg.get_ref().clone();
  let user_id: Uuid = match session.get(SESSION_ID_KEY) {
    Ok(Some(id)) => id,
    _ => {
      let id = Uuid::new_v4();
      session.set(SESSION_ID_KEY, id).unwrap_or_default();
      id
    }
  };
  let user_profile = {{crate_name}}_service::profile::load(app_cfg.files(), user_id);
  let logger = app_cfg
    .root_logger()
    .new(slog::o!("action" => action_key, "user_id" => user_profile.name().clone()));
  let flash: Option<(String, String)> = match session.get(FLASH_KEY) {
    Ok(f) => f.map(|t: String| match t.find(':') {
      Some(idx) => (t[..idx].into(), t[(idx + 1)..].into()),
      None => ("success".into(), t.clone())
    }),
    Err(e) => {
      slog::warn!(logger, "Unable to read from session: {}", e);
      None
    }
  };
  let _ = flash.clone().map(|_| session.remove(FLASH_KEY));
  RequestContext::new(app_cfg, user_id, user_profile, logger, flash)
}

/// Set a session entry for flash messages. Allowed keys are "success", "warning", and "error". Values are used once, then unset.
pub fn add_flash(s: &Session, logger: &slog::Logger, key: &str, msg: &str) {
  match s.set(FLASH_KEY, format!("{}:{}", key, msg)) {
    Ok(_) => (),
    Err(e) => slog::warn!(logger, "Unable to write to session: {}", e)
  };
}

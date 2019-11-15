use slog;

use crate::cfg::AppConfig;
use {{crate_name}}_core::profile::UserProfile;
use {{crate_name}}_core::Result;

/// Provides reverse routing of urls
pub trait Router {
  fn route(&self, path: &str, args: &[&str]) -> Result<String>;
  fn route_simple(&self, path: &str) -> Result<String> {
    const NO_PARAMS: [&str; 0] = [];
    self.route(path, &NO_PARAMS)
  }
  fn route_static(&self, path: &str) -> Result<String> {
    self.route("static", &[&path])
  }
}

/// Contains information about the application and current user, along with common components
pub struct RequestContext {
  app: AppConfig,
  user_id: uuid::Uuid,
  user_profile: UserProfile,
  flash: Option<(String, String)>,
  log: slog::Logger
}

impl std::fmt::Debug for RequestContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RequestContext [{}] [{:?}]", self.user_id, self.user_profile)
  }
}

impl RequestContext {
  pub fn new(
    app: AppConfig,
    user_id: uuid::Uuid,
    user_profile: UserProfile,
    log: slog::Logger,
    flash: Option<(String, String)>
  ) -> RequestContext {
    let log = log.new(slog::o!("user_id" => user_id.to_string()));
    RequestContext {
      app,
      user_id,
      user_profile,
      flash,
      log
    }
  }

  pub fn app(&self) -> &AppConfig {
    &self.app
  }

  pub fn user_id(&self) -> &uuid::Uuid {
    &self.user_id
  }

  pub fn user_profile(&self) -> &UserProfile {
    &self.user_profile
  }

  pub fn flash(&self) -> &Option<(String, String)> {
    &self.flash
  }

  pub fn log(&self) -> &slog::Logger {
    &self.log
  }
}

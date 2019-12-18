use {{crate_name}}_core::profile::Theme;
use serde::Deserialize;

/// Transport class for editing your profile
#[derive(Debug, Deserialize)]
pub struct ProfileForm {
  username: String,
  theme: String,
  navbar_color: String,
  link_color: String
}

impl ProfileForm {
  pub const fn username(&self) -> &String {
    &self.username
  }

  pub fn theme(&self) -> Theme {
    match self.theme.parse::<Theme>() {
      Ok(t) => t,
      Err(_) => Theme::Light
    }
  }

  pub const fn navbar_color(&self) -> &String {
    &self.navbar_color
  }

  pub const fn link_color(&self) -> &String {
    &self.link_color
  }
}

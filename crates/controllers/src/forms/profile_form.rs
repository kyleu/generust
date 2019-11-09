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
  pub fn username(&self) -> &String {
    &self.username
  }

  pub fn theme(&self) -> {{crate_name}}_core::profile::Theme {
    match self.theme.parse::<{{crate_name}}_core::profile::Theme>() {
      Ok(t) => t,
      Err(_) => {{crate_name}}_core::profile::Theme::Light
    }
  }

  pub fn navbar_color(&self) -> &String {
    &self.navbar_color
  }

  pub fn link_color(&self) -> &String {
    &self.link_color
  }
}

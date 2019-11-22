
use serde::{Deserialize, Serialize};

/// The user's theme, initially just light and dark
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Theme {
  Light,
  Dark
}

impl std::str::FromStr for Theme {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> anyhow::Result<Self> {
    match s {
      "Light" => Ok(Theme::Light),
      "Dark" => Ok(Theme::Dark),
      _ => Err(anyhow::anyhow!("Invalid theme [{}]", s))
    }
  }
}

impl std::fmt::Display for Theme {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Theme::Light => "Light",
      Theme::Dark => "Dark"
    };
    write!(f, "{}", s)
  }
}

impl Theme {
  pub fn all() -> Vec<Theme> {
    vec![Theme::Light, Theme::Dark]
  }

  pub fn default_navbar_color(&self) -> String {
    match self {
      Theme::Light => "#263238".into(),
      Theme::Dark => "#263238".into()
    }
  }

  pub fn body_class(&self) -> &str {
    match self {
      Theme::Light => "uk-dark",
      Theme::Dark => "uk-light"
    }
  }

  pub fn card_class(&self) -> &str {
    match self {
      Theme::Light => "uk-card-default",
      Theme::Dark => "uk-card-secondary"
    }
  }
}

/// User preferences
#[derive(Clone, derive_more::Constructor, Debug, Deserialize, Serialize)]
pub struct UserProfile {
  name: String,
  theme: Theme,
  nav_color: String,
  link_color: String
}

impl Default for UserProfile {
  fn default() -> UserProfile {
    UserProfile {
      name: "Guest".into(),
      theme: Theme::Light,
      nav_color: "bluegrey".into(),
      link_color: "bluegrey".into()
    }
  }
}

impl UserProfile {
  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn theme(&self) -> &Theme {
    &self.theme
  }

  pub fn nav_color(&self) -> &String {
    &self.nav_color
  }

  pub fn link_color(&self) -> &String {
    &self.link_color
  }

  pub fn link_class(&self) -> String {
    format!("{}-fg", &self.link_color)
  }
}

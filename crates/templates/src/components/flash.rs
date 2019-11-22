use anyhow::Result;
use maud::{html, Markup};

pub(crate) fn flash(k: &str, v: &str) -> Result<Markup> {
  let cls = match k {
    "status" => "uk-alert-primary",
    "success" => "uk-alert-success",
    "warning" => "uk-alert-warning",
    "error" => "uk-alert-danger",
    _ => ""
  };

  Ok(html! {
    div.alert-top.(cls) data-uk-alert? {
      a.uk-alert-close href="#" data-uk-close="" {}
      p { (v) }
    }
  })
}

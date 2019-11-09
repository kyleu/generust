use maud::{html, Markup, PreEscaped};

use crate::components;
use {{crate_name}}_core::Result;
use {{crate_name}}_service::RequestContext;

pub(crate) fn page(ctx: &RequestContext, title: &str, content: Markup) -> Result<Markup> {
  Ok(html! {
    (PreEscaped("<!DOCTYPE html>"))
    html lang="en" {
      (components::header::header(&ctx, &format!("{} - {}", title, {{crate_name}}_core::APPNAME))?)
      body.(ctx.user_profile().theme().body_class()) {
        (content)
      }
    }
  })
}

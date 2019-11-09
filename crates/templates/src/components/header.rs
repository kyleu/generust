use maud::{html, Markup};

use {{crate_name}}_core::Result;
use {{crate_name}}_service::RequestContext;

pub(crate) fn header(ctx: &RequestContext, title: &str) -> Result<Markup> {
  Ok(html! {
    head {
      title { (title) }
      meta charset="utf-8";
      meta http-equiv="X-UA-Compatible" content="IE=edge";
      meta name="viewport" content="width=device-width, initial-scale=1.0";

      meta property="og:title" content=(title);
      meta property="og:type" content="website";
      meta property="og:locale" content="en_US";

      @if ctx.app().verbose() {
        link rel="stylesheet" media="screen" href=(ctx.router().route_static("vendor/style.css")?);
        script src=(ctx.router().route_static("vendor/uikit.js")?) {}
        script src=(ctx.router().route_static("vendor/uikit-icons.js")?) {}
      } @else {
        link rel="stylesheet" media="screen" href=(ctx.router().route_static("vendor/style.min.css")?);
        script src=(ctx.router().route_static("vendor/uikit.min.js")?) {}
        script src=(ctx.router().route_static("vendor/uikit-icons.min.js")?) {}
      }
    }
  })
}

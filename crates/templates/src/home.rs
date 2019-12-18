use anyhow::Result;
use maud::{html, Markup};
use {{crate_name}}_service::{RequestContext, Router};

pub fn index(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  let content = html! {
    div.uk-text-center {
      h1.uk-heading-hero {
        "Welcome to {{project-name}}"
      }
    }
    (socket(ctx)?)
    (testbed_list(ctx, router)?)
    script src=(router.route_static("client.js")?) defer? {}
  };
  crate::section(ctx, router, "Home", &content)
}

fn socket(ctx: &RequestContext) -> Result<Markup> {
  Ok(crate::card(ctx, &html! {
    h4 { "WebSocket" }
    div {
      a href="" onclick=(crate::utils::onclick_event("send-ping", "", "")) { "Send Ping" }
    }
    div#socket-status { "Connecting..." }
    div#socket-results.uk-margin-top { }
  }))
}

fn testbed_list(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  let ts = vec![
    ("dump", "Dump", "Displays a bunch of info about the app"),
    ("gallery", "Gallery", "Tests front-end components"),
    ("prototype", "Prototype", "A sandbox to play around in"),
    ("scroll", "Scroll", "Scrolling test"),
    ("crash", "Crash", "Simulates a server error"),
  ];
  Ok(crate::card(ctx, &html! {
    h4 { "Testbeds" }
    table.uk-table.uk-table-divider {
      tbody {
        @for t in ts {
          tr {
            td { a.(ctx.user_profile().link_class()) href=(router.route("testbed.detail", &[t.0])?) { (t.1) } }
            td { (t.2) }
          }
        }
      }
    }
  }))
}

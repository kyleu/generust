use anyhow::Result;
use {{crate_name}}_service::{RequestContext, Router};
use maud::{html, Markup};

pub fn not_found(ctx: &RequestContext, router: &dyn Router, path: &str) -> Result<Markup> {
  let content = html! {
    div.uk-text-center {
      h1.uk-heading-hero {
        "404 Not Found"
      }
      div.uk-text-lead {
        (path)
      }
    }
  };
  crate::section(ctx, router, "Not Found", &content)
}

pub fn exception(ctx: &RequestContext, router: &dyn Router, e: &anyhow::Error) -> Result<Markup> {
  let content = html! {
    div.uk-text-center {
      h1.uk-heading-hero {
        (e.to_string())
      }
      div.uk-text-lead {
        @for ex in e.chain().skip(1) {
          div { (ex.to_string()) }
        }
      }
      div.uk-margin-top {
        div.uk-text-left {
          (crate::components::backtrace::to_html(e.backtrace()))
        }
      }
    }
  };
  crate::section(ctx, router, "Error", &content)
}

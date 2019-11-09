use maud::{html, Markup};

use {{crate_name}}_core::Result;
use {{crate_name}}_service::RequestContext;

pub fn not_found(ctx: &RequestContext, path: &str) -> Result<Markup> {
  let content = html! {
    div.uk-container {
      div.uk-section.uk-section-small {
        div.uk-container {
          div.uk-text-center {
            h1.uk-heading-hero {
              "404 Not Found"
            }
            div.uk-text-lead {
              (path)
            }
          }
        }
      }
    }
  };
  Ok(html! {
    (crate::simple(ctx, "Not Found", content)?)
  })
}

pub fn exception(ctx: &RequestContext, e: &{{crate_name}}_core::Error) -> Result<Markup> {
  let content = html! {
    div.uk-container {
      div.uk-section.uk-section-small {
        div.uk-container {
          div.uk-text-center {
            h1.uk-heading-hero {
              (e.to_string())
            }
            div.uk-text-lead {
              @for e in e.iter().skip(1) {
                div { (e.to_string()) }
              }
            }
            div.uk-margin-top {
              div.uk-text-left {
                @if let Some(backtrace) = e.backtrace() {
                  (crate::components::backtrace::to_html(backtrace))
                }
              }
            }
          }
        }
      }
    }
  };
  Ok(html! {
    (crate::simple(ctx, "Error", content)?)
  })
}

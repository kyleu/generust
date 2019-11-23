use anyhow::Result;
use maud::{html, Markup};
use {{crate_name}}_service::{RequestContext, Router};

pub fn settings(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  let content = html! {
    div.uk-section.uk-section-small {
      div.uk-container {
        div.uk-text-center {
          h1.uk-heading-hero {
            "Settings"
          }
          div.uk-text-lead {
            "A work in progress"
          }
          div.uk-margin-top {
            table.uk-table.uk-table-divider.uk-text-left {
              tbody {
                tr {
                  th { "Config Directory" }
                  td { (ctx.app().files().cfg_dir()) }
                }
              }
            }
          }
        }
      }
    }
  };
  crate::simple(ctx, router, "Settings", content)
}

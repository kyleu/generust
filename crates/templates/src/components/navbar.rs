use anyhow::Result;
use maud::{html, Markup};
use {{crate_name}}_service::{RequestContext, Router};

pub(crate) fn navbar(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  Ok(html! {
    header {
      div data-uk-sticky="sel-target: .uk-navbar-container; cls-active: data-uk-navbar-sticky" {
        nav#navbar.(format!("{}-bg", &ctx.user_profile().nav_color())).uk-navbar-container data-uk-navbar? {
          div.uk-navbar-left {
            a.uk-navbar-item.uk-logo href="/" {
              span data-uk-icon="icon: cog; ratio: 1.6" {}
            }
            a.uk-navbar-item.uk-logo.uk-margin-remove.uk-padding-remove href="/" {
              ({{crate_name}}_core::APPNAME)
            }
          }
          div.uk-navbar-right {
            ul.uk-navbar-nav {
              li {
                a href=(router.route_simple("profile")?) data-uk-icon="icon:user" title={ (ctx.user_profile().name()) " Profile" } { }
              }
              li {
                a href=(router.route_simple("admin.list")?) data-uk-icon="icon: settings" title="Settings" { }
              }
            }
          }
        }
      }
    }
  })
}

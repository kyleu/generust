use maud::{html, Markup};

use {{crate_name}}_core::Result;
use {{crate_name}}_service::{RequestContext, Router};

pub fn list(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  let content = html!(
    div.uk-container {
      div.uk-section.uk-section-small {
        div.uk-container.uk-container-small {
          (crate::components::card::card(ctx, html!(
            h3 { "Project Administration" }
            ul {
              li { a.(ctx.user_profile().link_class()) href=(router.route_simple("admin.connections")?) { "Connection List" } }
              li { a.(ctx.user_profile().link_class()) href=(router.route_simple("admin.settings")?) { "Edit Settings" } }
            }
          )))
        }
      }
    }
  );
  Ok(html! {
    (crate::simple(ctx, router, "Project Administration", content)?)
  })
}

pub fn connections(
  ctx: &RequestContext, router: &dyn Router, conns: Vec<uuid::Uuid>, channels: Vec<(String, Vec<uuid::Uuid>)>
) -> Result<Markup> {
  let conn_content = crate::components::card::card(ctx, html! {
    h3 { (format!("[{}] Connections", conns.len())) }
    ul {
      @for c in &conns {
        li { a.(ctx.user_profile().link_class()) href=(router.route("admin.connection_detail", &[&format!("{}", c)])?) { (format!("{}", c)) } }
      }
    }
  });
  let channel_content = crate::components::card::card(ctx, html! {
    h3 { (format!("[{}] Channels", channels.len())) }
    ul {
      @for c in &channels {
        li {
          (format!("{}: {} connections", c.0, c.1.len()))
          ul {
            @for id in &c.1 {
              li { a.(ctx.user_profile().link_class()) href=(router.route("admin.connection_detail", &[&format!("{}", id)])?) { (format!("{}", id)) } }
            }
          }
        }
      }
    }
  });

  let content = html!(
    div.uk-container {
      div.uk-section.uk-section-small {
        div.uk-container.uk-container-small {
          (conn_content)
          (channel_content)
        }
      }
    }
  );
  Ok(html! {
    (crate::simple(ctx, router, "Connection Listing", content)?)
  })
}

pub fn connection_detail(ctx: &RequestContext, router: &dyn Router, id: uuid::Uuid) -> Result<Markup> {
  let content = html!(
    div.uk-container {
      div.uk-section.uk-section-small {
        div.uk-container.uk-container-small {
          h3 { (format!("Connection [{}]", id)) }
          form.uk-form-stacked action="" method="post" {
            div.uk-margin-small {
              label.uk-form-label { "Send Message" }
              input.uk-input type="text" name="msg" placeholder="..." {}
            }
            div.uk-margin-small {
              button.uk-button.uk-button-default type="submit" { "Send" }
            }
          }
        }
      }
    }
  );
  Ok(html! {
    (crate::simple(ctx, router, "Connection Detail", content)?)
  })
}

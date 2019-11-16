use maud::{html, Markup};

use {{crate_name}}_core::Result;
use {{crate_name}}_service::{RequestContext, Router};

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
          a.(ctx.user_profile().link_class()) href=(router.route("admin.channel_detail", &[&format!("{}", c.0)])?) {
            (format!("{}", c.0))
          }
          (format!(": {} connections", c.1.len()))
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
    (conn_content)
    (channel_content)
  );
  crate::section(ctx, router, "Connection Listing", content)
}

pub fn connection_detail(ctx: &RequestContext, router: &dyn Router, id: uuid::Uuid) -> Result<Markup> {
  let content = html!(
    h3 { (format!("Connection [{}]", id)) }
    (send_form())
  );
  crate::section(ctx, router, "Connection Detail", content)
}

fn send_form() -> Markup {
  html!(
    form.uk-form-stacked action="" method="post" {
      div.uk-margin-small {
        label.uk-form-label { "Level" }
        select.uk-select name="level" {
          option value="success" { "Success" }
          option value="info" { "Info" }
          option value="warning" { "Warning" }
          option value="error" { "Error" }
        }
      }
      div.uk-margin-small {
        label.uk-form-label { "Message" }
        input.uk-input type="text" name="content" placeholder="..." {}
      }
      div.uk-margin-small {
        button.uk-button.uk-button-default type="submit" { "Send" }
      }
    }
  )
}

pub fn channel_detail(ctx: &RequestContext, router: &dyn Router, key: &str) -> Result<Markup> {
  let content = html!(
    h3 { (format!("Channel [{}]", key)) }
    (send_form())
  );
  crate::section(ctx, router, "Connection Detail", content)
}

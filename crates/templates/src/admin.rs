use anyhow::Result;
use maud::{html, Markup};
use {{crate_name}}_service::{RequestContext, Router};

pub fn list(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  let content = crate::components::card::card(
    ctx,
    html!(
      h3 { "Project Administration" }
      ul {
        li { a.(ctx.user_profile().link_class()) href=(router.route_simple("admin.connections")?) { "Connection List" } }
        li { a.(ctx.user_profile().link_class()) href=(router.route_simple("admin.settings")?) { "Edit Settings" } }
      }
    )
  );
  crate::section(ctx, router, "Project Administration", content)
}

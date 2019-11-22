use anyhow::Result;
use {{crate_name}}_service::{RequestContext, Router};
use {{crate_name}}_core::build_info;
use maud::{html, Markup};

fn container(ctx: &RequestContext, router: &dyn Router, result: &str, content: Markup) -> Result<Markup> {
  let content = html! {
    div.uk-text-center {
      h1.uk-heading-hero { "Testbed" }
      div.uk-text-lead { (result) }
    }
    div.uk-margin-top {
      (content)
    }
  };
  crate::section(ctx, router, "Testbed", content)
}

pub fn dump(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  let content = crate::card(ctx, html! {
    table.uk-table.uk-table-divider.uk-text-left {
      tbody {
        tr {
          th { "Config Dir" }
          td { (ctx.app().files().cfg_dir()) }
        }
        tr {
          th { "Version" }
          td { (build_info::PKG_VERSION) }
        }
        tr {
          th { "Project" }
          td { (format!("{}: {}", build_info::PKG_NAME, build_info::PKG_DESCRIPTION)) }
        }
        tr {
          th { "Website" }
          td { a.(ctx.user_profile().link_class()) href=(build_info::PKG_HOMEPAGE) target="_blank" { (build_info::PKG_HOMEPAGE) } }
        }
        tr {
          th { "Architecture" }
          td { (build_info::TARGET) }
        }
        tr {
          th { "Profile" }
          td { (format!("{} (opt {})", build_info::PROFILE, build_info::OPT_LEVEL)) }
        }
        tr {
          th { "Built At" }
          td { (build_info::BUILT_TIME_UTC) }
        }
        tr {
          th { "CPU Count" }
          td { (format!("{}", num_cpus::get())) }
        }
      }
    }
  });
  container(ctx, router, "Dump", content)
}

pub fn gallery(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  let content = crate::gallery::gallery(&ctx);
  container(ctx, router, "Gallery", content)
}

pub fn prototype(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  let content = html! { "Prototype!" };
  crate::components::page::page(ctx, router, "Prototype!", content)
}

pub fn scroll(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  let content = html! {
    div style="height: 2048px;" {
      "Hello!"
      " This is a tall block, test scrolling if you'd like "
    }
  };
  container(ctx, router, "Scroll", content)
}

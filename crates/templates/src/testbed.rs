use maud::{html, Markup};

use {{crate_name}}_core::Result;
use {{crate_name}}_service::RequestContext;

use {{crate_name}}_core::build_info;

fn container(ctx: &RequestContext, result: &str, content: Markup) -> Result<Markup> {
  let content = html! {
    div.uk-container {
      div.uk-section.uk-section-small {
        div.uk-container {
          div.uk-text-center {
            h1.uk-heading-hero { "Testbed" }
            div.uk-text-lead { (result) }
          }
          div.uk-margin-top {
            (content)
          }
        }
      }
    }
  };
  Ok(html! {
    (crate::simple(ctx, "Testbed", content)?)
  })
}

pub fn dump(ctx: &RequestContext) -> Result<Markup> {
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
  Ok(html! {
    (container(ctx, "Dump", content)?)
  })
}

pub fn gallery(ctx: &RequestContext) -> Result<Markup> {
  let content = crate::gallery::gallery(&ctx);
  Ok(html! {
    (container(ctx, "Gallery", content)?)
  })
}

pub fn prototype(ctx: &RequestContext) -> Result<Markup> {
  let content = html! { "Prototype!" };
  Ok(html! {
    (crate::components::page::page(ctx, "Prototype!", content)?)
  })
}

pub fn scroll(ctx: &RequestContext) -> Result<Markup> {
  let content = html! {
    div style="height: 2048px;" {
      "Hello!"
      " This is a tall block, test scrolling if you'd like "
    }
  };
  Ok(html! {
    (container(ctx, "Scroll", content)?)
  })
}

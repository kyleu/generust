use maud::{html, Markup};

use {{crate_name}}_core::Result;
use {{crate_name}}_service::RequestContext;

pub(crate) fn simple(ctx: &RequestContext, title: &str, content: Markup) -> Result<Markup> {
  Ok(crate::components::page::page(ctx, title, html! {
    (crate::components::navbar::navbar(&ctx)?)

    div#content data-uk-height-viewport="expand: true" {
      @if let Some(f) = ctx.flash() {
        (crate::components::flash::flash(&f.0, &f.1)?)
      }
      (content)
    }
  })?)
}

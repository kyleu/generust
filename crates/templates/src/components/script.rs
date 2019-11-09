use maud::{html, Markup, PreEscaped};

pub(crate) fn script_inline(content: &str) -> Markup {
  html! {
    script { (PreEscaped(content)) }
  }
}

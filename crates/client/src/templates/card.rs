use maud::{html, Markup};

// Unused, rename if you need it
pub(crate) fn _card(ctx: &crate::ctx::ClientContext, content: Markup) -> Markup {
  html! {
    div.uk-margin.uk-card.uk-card-body.(ctx.profile().theme().card_class()) {
      (content)
    }
  }
}

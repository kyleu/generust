use maud::{html, Markup};

use {{crate_name}}_core::profile::Theme;
use {{crate_name}}_core::Result;
use {{crate_name}}_service::{RequestContext, Router};

pub fn profile(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  let content = html! {
    div.uk-section.uk-section-small {
      div.uk-container {
        div.uk-text-center {
          h1.uk-heading-hero {
            "Profile"
          }
        }
        div.uk-margin-top.uk-container.uk-container-small {
          (crate::card(&ctx, html! {
            form action="#" method="post" {
              fieldset.uk-fieldset {
                div.uk-margin {
                  label for="username" { "Username" }
                  input.uk-input#username name="username" type="text" value=(ctx.user_profile().name());
                }
                div.uk-margin {
                  div { label { "Theme" } }
                  @for t in Theme::all() {
                    label for=(format!("theme-{}", t)) {
                      input.uk-radio#(format!("theme-{}", t)) checked?[ctx.user_profile().theme() == &t] name="theme" type="radio" value=(format!("{}", t));
                      " " (format!("{}", t)) " "
                    }
                  }
                }

                input#navbar_color name="navbar_color" type="hidden" value=(ctx.user_profile().nav_color());
                div.uk-margin {
                  label { "Navbar Color" }
                  div#colors {
                    @for c in crate::components::colors::COLORS.iter() {
                      (nav_swatch(ctx.user_profile(), c))
                    }
                  }
                }

                input#link_color name="link_color" type="hidden" value=(ctx.user_profile().link_color());
                div.uk-margin {
                  label {
                    a.(ctx.user_profile().link_class()) id="link_example" href="" onclick="return false;" { "Link" }
                    " Color"
                  }
                  div#colors {
                    @for c in crate::components::colors::COLORS.iter() {
                      (link_swatch(ctx.user_profile(), c))
                    }
                  }
                }

                div {
                  button.uk-button.uk-button-default type="submit" { "Save Changes" }
                }
              }
            }
          }))
        }
      }
    }
    (crate::components::script::script_inline(r#"
      function nav_color(el, c) {
        var input = document.getElementById("navbar_color");
        input.value = c;
        var nb = document.getElementById("navbar");
        nb.className = (c + "-bg data-uk-navbar-container data-uk-navbar");
        var colors = document.querySelectorAll(".nav_swatch");
        colors.forEach(function(i) {
          i.classList.remove("active");
        })
        el.classList.add("active");
      }

      function link_color(el, c) {
        var input = document.getElementById("link_color");
        input.value = c;
        var l = document.getElementById("link_example");
        l.className = (c + "-fg");
        var colors = document.querySelectorAll(".link_swatch");
        colors.forEach(function(i) {
          i.classList.remove("active");
        })
        el.classList.add("active");
      }
    "#))
  };
  Ok(html! {
    (crate::simple(ctx, router, "Profile", content)?)
  })
}

fn nav_swatch(p: &{{crate_name}}_core::profile::UserProfile, c: &str) -> Markup {
  let cls = if p.nav_color() == c { "active" } else { "" };
  html! {
    div.swatch.nav_swatch.(cls).(format!("{}-bg", c)).uk-text-center title=(c) onclick=(format!("nav_color(this, '{}')", c)) {
      div.icon data-uk-icon="icon: check" {}
    }
  }
}

fn link_swatch(p: &{{crate_name}}_core::profile::UserProfile, c: &str) -> Markup {
  let cls = if p.link_color() == c { "active" } else { "" };
  html! {
    div.swatch.link_swatch.(cls).(format!("{}-bg", c)).uk-text-center title=(c) onclick=(format!("link_color(this, '{}')", c)) {
      div.icon data-uk-icon="icon: check" {}
    }
  }
}

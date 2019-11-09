use maud::{html, Markup};

use {{crate_name}}_service::RequestContext;

pub(crate) fn gallery(ctx: &RequestContext) -> Markup {
  html! {
    (navbar(ctx))
    div.uk-section {
      div.uk-container {
        div data-uk-grid? {
          div.("uk-width-2-3@m") {
            (breadcrumbs())
            article.uk-article {
              (article())
              hr.uk-divider-icon.uk-margin-medium;
              (buttons())
              hr.uk-margin-medium;
              (text())
              pre.uk-pre.uk-margin-medium {
                code { r#"<div class="myclass">...</div>"# }
              }
              (blockquote())
            }
            hr.uk-margin-medium;
            (comments())
            (pagination())
          }
          div.("uk-width-expand@m") {
            (search())
            (navlist())
            (cards(&ctx))
          }
        }
        hr.uk-margin-large;
        div.uk-grid-divider data-uk-grid="" {
          div.("uk-width-3-5@m") {
            div.("uk-child-width-expand@s") data-uk-grid="" {
              (forms())
            }
            div.uk-overflow-auto.uk-margin-medium-top {
              (table())
            }
            (alerts())
            (countdown())
          }
          div.("uk-width-2-5@m") {
            (text_sizes())
          }
        }
        hr.uk-margin-medium;
        (images())
        hr.uk-margin-medium;
        (widgets())
        hr.uk-margin-medium;
        (tabs())
        hr.uk-margin-medium;
        (lists())
      }
      div.uk-section.uk-padding-remove-vertical {
        (grid())
      }
      (sections())
    }
    (modals())
    (offcanvas())
  }
}

fn navbar(ctx: &RequestContext) -> Markup {
  html! {
    nav.uk-navbar-container.(format!("{}-bg", ctx.user_profile().nav_color())) {
      div.uk-container {
        div data-uk-navbar? {
          div.uk-navbar-left {
            a.uk-navbar-item.uk-logo href="#" { "Logo" }
            ul.uk-navbar-nav {
              li.uk-active {
                a href="#" { "Active"}
              }
              li {
                a href="#" { "Parent" }
                div.uk-navbar-dropdown {
                  ul.uk-nav.uk-navbar-dropdown-nav {
                    li.uk-active {
                      a href="#" { "Active" }
                    }
                    li.uk-parent {
                      a href="#" { "Parent" }
                      ul.uk-nav-sub {
                        li {
                          a href="#" { "Sub item" }
                        }
                        li {
                          a href="#" { "Sub item" }
                        }
                      }
                    }
                    li.uk-nav-header { "Header" }
                    li {
                      a href="#" {
                        span.uk-margin-small-right data-uk-icon="icon: table" {}
                        "Item"
                      }
                    }
                    li {
                      a href="#" {
                        span.uk-margin-small-right data-uk-icon="icon: thumbnails" {}
                        "Item"
                      }
                    }
                    li.uk-nav-divider {}
                    li {
                      a href="#" {
                        span.uk-margin-small-right data-uk-icon="icon: trash" {}
                        "Item"
                      }
                    }
                  }
                }
              }
              li {
                a href="#" { "Item" }
              }
              li {
                a href="#" { "Item" }
              }
            }
          }
          div.uk-navbar-right {
            a.uk-navbar-toggle href="#modal" data-uk-icon="icon: more-vertical" data-uk-toggle? {}
            a.uk-navbar-toggle href="#modal-search" data-uk-search-icon="" data-uk-toggle? {}
            a.uk-navbar-toggle href="#offcanvas" data-uk-navbar-toggle-icon="" data-uk-toggle? {}
          }
        }
      }
    }
  }
}

fn breadcrumbs() -> Markup {
  html! {
    ul.uk-breadcrumb {
      li {
        a href="#" { "Home" }
      }
      li {
        a href="#" { "Blog" }
      }
      li.uk-disabled {
        a { "Category" }
      }
      li {
        span { "Post" }
      }
    }
  }
}

fn article() -> Markup {
  html! {
    h1.uk-article-title {
      a.uk-link-reset href="#" { "Article Title" }
    }
    hr.uk-divider-small;
    p.uk-text-lead {
      "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
    }
    p.uk-dropcap.("uk-column-1-2@s") {
      "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
    }
    p.uk-article-meta {
      "Written by "
      a href="#" { "Super User" }
      " on 12 April 2012. Posted in "
      a href="#" { "Blog" }
    }
  }
}

fn buttons() -> Markup {
  html! {
    .uk-grid-small.uk-child-width-auto.uk-flex-middle.uk-margin-medium data-uk-grid? {
      div {
        button.uk-button.uk-button-default { "Default" }
        div data-uk-dropdown="mode: click" {
          ul.uk-nav.uk-dropdown-nav {
            li.uk-active {
              a href="#" { "Active" }
            }
            li.uk-parent {
              a href="#" { "Parent" }
              ul.uk-nav-sub {
                li {
                  a href="#" { "Sub item" }
                }
                li {
                  a href="#" { "Sub item" }
                }
              }
            }
            li.uk-nav-header { "Header" }
            li {
              a href="#" {
                span.uk-margin-small-right data-uk-icon="icon: table" {}
                "Item"
              }
            }
            li {
              a href="#" {
                span.uk-margin-small-right data-uk-icon="icon: thumbnails" {}
                "Item"
              }
            }
            li.uk-nav-divider {}
            li {
              a href="#" {
                span.uk-margin-small-right data-uk-icon="icon: trash" {}
                "Item"
              }
            }
          }
        }
      }
      div {
        button.uk-button.uk-button-primary { "Primary" }
      }
      div {
        button.uk-button.uk-button-secondary { "Secondary" }
      }
      div {
        button.uk-button.uk-button-danger { "Danger" }
      }
      div {
        button.uk-button.uk-button-default disabled="" { "Disabled" }
      }
    }
  }
}

fn text() -> Markup {
  html! {
    div.("uk-child-width-1-2").uk-margin.("uk-child-width-1-4@s") data-uk-grid="" {
      div {
        ul.uk-list {
          li {
            a href="#" { "a element" }
          }
          li {
            abbr title="Title text" { "abbr element" }
          }
          li {
            code { "code element" }
          }
          li {
            del { "del element" }
          }
          li {
            dfn title="Title text" { "dfn element" }
          }
          li {
            a.uk-link-muted href="#" { "Link Muted" }
          }
        }
      }
      div {
        ul.uk-list {
          li {
            em { "em element" }
          }
          li {
            ins { "ins element" }
          }
          li {
            mark { "mark element" }
          }
          li {
            q {
              "q"
              q { "inside" }
              "a q"
            }
          }
          li {
            strong { "strong element" }
          }
          li {
            a.uk-link-reset href="#" { "Link Reset" }
          }
        }
      }
      div {
        ul.uk-list {
          li.uk-text-muted { "Text Muted" }
          li.uk-text-emphasis { "Text Emphasis" }
          li.uk-text-primary { "Text Primary" }
          li.uk-text-secondary { "Text Secondary" }
          li.uk-text-success { "Text Success" }
          li.uk-text-warning { "Text Warning" }
          li.uk-text-danger { "Text Danger" }
          li.uk-text-meta { "Text Meta" }
        }
      }
      div {
        ul.uk-list {
          li {
            span.uk-label { "Default" }
          }
          li {
            span.uk-label.uk-label-success { "Success" }
          }
          li {
            span.uk-label.uk-label-warning { "Warning" }
          }
          li {
            span.uk-label.uk-label-danger { "Danger" }
          }
          li {
            a.uk-badge href="#" { "1" }
          }
          li {
            a.uk-icon-button href="#" data-uk-icon="icon: home" {}
            a.uk-icon-button href="#" data-uk-icon="icon: github" {}
            a.uk-icon-link href="#" data-uk-icon="icon: trash" {}
          }
        }
      }
    }
  }
}

fn blockquote() -> Markup {
  html! {
    blockquote.uk-margin-medium cite="#" {
      p {
        "The blockquote element represents content that is quoted from another source, optionally with a citation which must be within a footer or cite element."
      }
      footer {
        "Someone famous in "
        cite {
          a href="#" { "Source Title" }
        }
      }
    }
    div.uk-grid-small data-uk-grid? {
      div {
        a.uk-button.uk-button-text href="#" { "Read more" }
      }
      div {
        a.uk-button.uk-button-text href="#" { "5 Comments" }
      }
    }
  }
}

fn comments() -> Markup {
  html! {
    ul.uk-comment-list.uk-margin-medium {
      li {
        article.uk-comment.uk-visible-toggle tabindex="-1" {
          header.uk-comment-header.uk-position-relative {
            .uk-grid-medium.uk-flex-middle data-uk-grid? {
              .uk-width-auto {
                img.uk-comment-avatar src="https://placekitten.com/50/50" width="50" alt="" {}
              }
              .uk-width-expand {
                h4.uk-comment-title.uk-margin-remove {
                  a.uk-link-reset href="#" { "Author" }
                }
                p.uk-comment-meta.uk-margin-remove-top {
                  a.uk-link-reset href="#" { "12 days ago" }
                }
              }
            }
            div.uk-position-top-right.uk-position-small.uk-hidden-hover {
              a.uk-button.uk-button-text href="#" { "Reply" }
            }
          }
          div.uk-comment-body {
            p {
              "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua."
            }
          }
        }
      }
    }
  }
}

fn pagination() -> Markup {
  html! {
    ul.uk-pagination.uk-flex-center data-uk-margin? {
      li {
        a href="#" {
          span data-uk-pagination-previous? {}
        }
      }
      li {
        a href="#" { "1" }
      }
      li.uk-disabled {
        span { "..." }
      }
      li {
        a href="#" { "4" }
      }
      li {
        a href="#" { "5" }
      }
      li {
        a href="#" { "6" }
      }
      li.uk-active {
        span { "7" }
      }
      li {
        a href="#" { "8" }
      }
      li {
        a href="#" { "9" }
      }
      li {
        a href="#" { "10" }
      }
      li.uk-disabled {
        span { "..." }
      }
      li {
        a href="#" { "20" }
      }
      li {
        a href="#" {
          span data-uk-pagination-next="" {}
        }
      }
    }
  }
}

fn search() -> Markup {
  html! {
    div.uk-margin-medium-bottom {
      form.uk-search.uk-search-default.("uk-width-1-1") {
        span data-uk-search-icon="" {}
        input.uk-search-input type="search" placeholder="Search..." {}
      }
    }
  }
}

fn navlist() -> Markup {
  html! {
    ul.uk-nav-default.uk-nav-parent-icon.uk-margin-medium data-uk-nav? {
      li.uk-active {
        a href="#"  { "Active" }
      }
      li.uk-parent {
        a href="#" { "Parent" }
        ul.uk-nav-sub {
          li {
            a href="#" { "Sub item" }
          }
          li {
            a href="#" { "Sub item" }
            ul {
              li {
                a href="#" { "Sub item" }
              }
              li {
                a href="#" { "Sub item" }
              }
            }
          }
        }
      }
      li.uk-parent {
        a href="#" { "Parent" }
        ul.uk-nav-sub {
          li {
            a href="#" { "Sub item" }
          }
          li {
            a href="#" { "Sub item" }
          }
        }
      }
      li.uk-nav-header { "Header" }
      li {
        a href="#" {
          span.uk-margin-small-right data-uk-icon="icon: table" {}
          "Item"
        }
      }
      li {
        a href="#" {
          span.uk-margin-small-right data-uk-icon="icon: thumbnails" {}
          "Item"
        }
      }
      li.uk-nav-divider {}
      li {
        a href="#" {
          span.uk-margin-small-right data-uk-icon="icon: trash" {}
          "Item"
        }
      }
    }
  }
}

fn cards(ctx: &RequestContext) -> Markup {
  html! {
    (crate::card(&ctx, html! {
      h3.uk-card-title { "Default" }
      p {
        "Lorem ipsum "
        a href="#" { "dolor" }
        " sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
      }
    }))
    (crate::card(&ctx, html! {
      h3.uk-card-title { "Primary" }
      p {
        "Lorem ipsum "
        a href="#" { "dolor" }
        " sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
      }
    }))
    (crate::card(&ctx, html! {
      h3.uk-card-title { "Secondary" }
      p {
        "Lorem ipsum "
        a href="#" { "dolor" }
        " sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
      }
    }))
    (crate::card(&ctx, html! {
      h3.uk-card-title { "Hover" }
      p {
        "Lorem ipsum "
        a href="#" { "dolor" }
        " sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
      }
    }))
  }
}

fn forms() -> Markup {
  html! {
    div {
      form.uk-form-stacked {
        div.uk-margin-small {
          label.uk-form-label { "Text" }
          input.uk-input type="text" placeholder="Some text..." {}
        }
        div.uk-margin-small {
          select.uk-select {
            option { "Option 01" }
            option { "Option 02" }
          }
        }
        div.uk-margin-small {
          textarea.uk-textarea rows="2" placeholder="Some text..." {}
        }
        div.uk-grid-small.uk-child-width-auto data-uk-grid? {
          div {
            label {
              input.uk-radio type="radio" name="radio" {}
              " Radio"
            }
          }
          div {
            label {
              input.uk-checkbox type="checkbox" {}
              " Checkbox"
            }
          }
        }
        div.uk-margin-small {
          input.uk-range type="range" value="2" min="0" max="10" step="0.1" {}
        }
      }
    }
    div {
      form.uk-form-stacked {
        div.uk-margin-small {
          label.uk-form-label { "States" }
          input.uk-input type="text" placeholder=":disabled" disabled="" {}
        }
        div.uk-margin-small {
          input.uk-input.uk-form-danger type="text" placeholder="form-danger" value="form-danger" {}
        }
        div.uk-margin-small {
          input.uk-input.uk-form-success type="text" placeholder="form-success" value="form-success" {}
        }
        div.uk-margin-small {
          input.uk-input.uk-form-blank type="text" placeholder="form-blank" {}
        }
      }
    }
  }
}

fn table() -> Markup {
  html! {
    table.uk-table.uk-table-divider.uk-table-hover.uk-table-small {
      thead {
        tr {
          th { "Table Heading" }
          th { "Table Heading" }
          th { "Table Heading" }
        }
      }
      tbody {
        tr {
          td { "Table Data" }
          td { "Table Data" }
          td { "Table Data" }
        }
        tr {
          td { "Table Data" }
          td { "Table Data" }
          td { "Table Data" }
        }
        tr {
          td { "Table Data" }
          td { "Table Data" }
          td { "Table Data" }
        }
        tr {
          td { "Table Data" }
          td { "Table Data" }
          td { "Table Data" }
        }
      }
    }
  }
}

fn alerts() -> Markup {
  html! {
    div.uk-margin-medium-top data-uk-alert? {
      a.uk-alert-close href="#" data-uk-close="" {}
      p { "Default" }
    }
    div.uk-alert-primary data-uk-alert? {
      a.uk-alert-close href="#" data-uk-close="" {}
      p { "Primary" }
    }
    div.uk-alert-success data-uk-alert? {
      a.uk-alert-close href="#" data-uk-close="" {}
      p { "Success" }
    }
    div.uk-alert-warning data-uk-alert? {
      a.uk-alert-close href="#" data-uk-close="" {}
      p { "Warning" }
    }
    div.uk-alert-danger.uk-margin-remove-bottom data-uk-alert? {
      a.uk-alert-close href="#" data-uk-close="" {}
      p { "Danger" }
    }
  }
}

fn countdown() -> Markup {
  html! {
    div.uk-grid-small.uk-child-width-auto.uk-margin-medium-top data-uk-grid? js-countdown? {
      div {
        div.uk-countdown-number.uk-countdown-days {}
      }
      div.uk-countdown-separator { ":" }
      div {
        div.uk-countdown-number.uk-countdown-hours {}
      }
      div.uk-countdown-separator { ":" }
      div {
        div.uk-countdown-number.uk-countdown-minutes {}
      }
      div.uk-countdown-separator { ":" }
      div {
        div.uk-countdown-number.uk-countdown-seconds {}
      }
    }
    script { r#"
      var date = (new Date(Date.now() + 864e5 * 7)).toISOString();
      UIkit.util.$$('[js-countdown]').forEach(function(el) {
        UIkit.countdown(el, {date: date});
      });
    "# }
  }
}

fn text_sizes() -> Markup {
  html! {
    h1.("uk-heading-2xlarge").uk-margin-small { "2XL" }
    h1.uk-heading-xlarge.uk-margin-small { "X-Large" }
    h1.uk-heading-large.uk-margin-small { "Heading L" }
    h1.uk-heading-medium.uk-margin-small { "Heading M" }
    h1.uk-heading-small.uk-margin-small { "Heading S" }
    h1.uk-margin-small { "Heading H1" }
    h2.uk-margin-small { "Heading H2" }
    h3.uk-margin-small { "Heading H3" }
    h4.uk-margin-small { "Heading H4" }
    h5.uk-margin-small { "Heading H5" }
    h6.uk-margin-small { "Heading H6" }
    h3.uk-heading-divider { "Heading Divider" }
    h3.uk-heading-bullet { "Heading Bullet" }
    h3.uk-heading-line {
      span { "Heading Line" }
    }
  }
}

fn images() -> Markup {
  html! {
    div.("uk-child-width-1-2@s").("uk-child-width-expand@m") data-uk-grid? {
      div {
        div.uk-inline {
          img src="https://placekitten.com/300/200" alt="" {}
          a.uk-position-absolute.uk-transform-center style="left: 20%; top: 30%" href="#" data-uk-marker="" {}
          a.uk-position-absolute.uk-transform-center style="left: 60%; top: 40%" href="#" data-uk-marker="" {}
          a.uk-position-absolute.uk-transform-center style="left: 80%; top: 70%" href="#" data-uk-marker="" {}
        }
      }
      div {
        div.uk-inline-clip {
          img src="https://placekitten.com/300/200" alt="" {}
          div.uk-overlay.uk-position-bottom {
            p { "Lorem ipsum dolor sit amet, consectetur." }
          }
        }
      }
      div {
        div.uk-inline-clip {
          img src="https://placekitten.com/300/200" alt="" {}
          div.uk-overlay.uk-position-bottom {
            p { "Lorem ipsum dolor sit amet, consectetur." }
          }
        }
      }
      div {
        div.uk-inline.uk-light {
          img src="https://placekitten.com/300/200" alt="" {}
          div.uk-position-center {
            span data-uk-overlay-icon? {}
          }
        }
      }
    }
  }
}

fn widgets() -> Markup {
  html! {
    div.uk-grid-divider.("uk-child-width-auto@m") data-uk-grid? {
      div {
        ul.uk-dotnav {
          li.uk-active {
            a href="#" { "Item 1" }
          }
          li {
            a href="#" { "Item 2" }
          }
          li {
            a href="#" { "Item 3" }
          }
          li {
            a href="#" { "Item 4" }
          }
          li {
            a href="#" { "Item 5" }
          }
          li {
            a href="#" { "Item 6" }
          }
        }
      }
      div {
        a href="#" data-uk-slidenav-previous? {}
        a href="#" data-uk-slidenav-next? {}
      }
      div {
        ul.uk-thumbnav {
          li.uk-active {
            a href="#" {
              img src="https://placekitten.com/300/200" width="60" alt="";
            }
          }
          li {
            a href="#" {
              img src="https://placekitten.com/300/200" width="60" alt="";
            }
          }
          li {
            a href="#" {
              img src="https://placekitten.com/300/200" width="60" alt="";
            }
          }
        }
      }
      div {
        div.uk-tooltip.uk-tooltip-top-center.uk-display-inline-block.uk-margin-remove.uk-position-relative { "Tooltip" }
      }
      div.("uk-width-expand@m") {
        progress.uk-progress value="45" max="100" { "45%" }
      }
      div {
        button type="button" data-uk-close="" {}
      }
      div {
        a href="#" data-uk-totop="" {}
      }
    }
  }
}

fn tabs() -> Markup {
  html! {
    div.uk-grid-divider.("uk-child-width-expand@m") data-uk-grid? {
      div {
        ul.uk-subnav.uk-subnav-divider data-uk-margin? {
          li.uk-active {
            a href="#" { "Active" }
          }
          li {
            a href="#" { "Item" }
          }
          li.uk-disabled {
            a { "Disabled" }
          }
        }
      }
      div {
        ul.uk-subnav.uk-subnav-pill data-uk-margin? {
          li.uk-active {
            a href="#" { "Active" }
          }
          li {
            a href="#" { "Item" }
          }
          li.uk-disabled {
            a { "Disabled" }
          }
        }
      }
      div {
        ul data-uk-tab? {
          li.uk-active {
            a href="#" { "Active" }
          }
          li {
            a href="#" { "Item" }
          }
          li.uk-disabled {
            a { "Disabled" }
          }
        }
      }
    }
  }
}

fn lists() -> Markup {
  html! {
    div.uk-grid-divider.("uk-child-width-expand@m") data-uk-grid? {
      div {
        ul.uk-list.uk-list-bullet.uk-margin-medium {
          li { "List item 1" }
          li { "List item 2" }
          li { "List item 3" }
        }
        ul.uk-list.uk-list-striped {
          li { "List item 1" }
          li { "List item 2" }
          li { "List item 3" }
        }
      }
      div {
        ul.uk-list.uk-list-divider.uk-margin-medium {
          li { "List item 1" }
          li { "List item 2" }
          li { "List item 3" }
        }
        dl.uk-description-list.uk-description-list-divider {
          dt { "Description lists" }
          dd { "A description text" }
          dt { "Description lists" }
          dd { "A description text" }
        }
      }
      div {
        ul data-uk-accordion="" {
          li.uk-open {
            a.uk-accordion-title href="#" { "Item 1" }
            div.uk-accordion-content {
              p {
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco."
              }
            }
          }
          li {
            a.uk-accordion-title href="#" { "Item 2" }
            div.uk-accordion-content {
              p {
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco."
              }
            }
          }
          li {
            a.uk-accordion-title href="#" { "Item 3" }
            div.uk-accordion-content {
              p {
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco."
              }
            }
          }
        }
      }
    }
  }
}

fn grid() -> Markup {
  html! {
    div.uk-grid-collapse.uk-grid-match.("uk-child-width-1-2@s").("uk-child-width-1-4@l") data-uk-grid? {
      div {
        div.uk-tile { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor." }
      }
      div {
        div.uk-tile { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor." }
      }
      div {
        div.uk-tile { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor." }
      }
      div {
        div.uk-tile { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor." }
      }
    }
  }
}

fn sections() -> Markup {
  html! {
    div.uk-section {
      div.uk-container {
        div.uk-grid-large.uk-flex-middle data-uk-grid? {
          div.("uk-width-expand@m") {
            p.uk-text-large { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor." }
          }
          div.("uk-width-auto@m") {
            a.uk-button.uk-button-default.uk-button-large href="#" { "Button" }
          }
        }
      }
    }
    div.uk-section {
      div.uk-container {
        div.uk-grid-large.uk-flex-middle data-uk-grid? {
          div.("uk-width-expand@m") {
            p.uk-text-large { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor." }
          }
          div.("uk-width-auto@m") {
            a.uk-button.uk-button-default.uk-button-large href="#" { "Button" }
          }
        }
      }
    }
    div.uk-section {
      div.uk-container {
        div.uk-grid-large.uk-flex-middle data-uk-grid? {
          div.("uk-width-expand@m") {
            p.uk-text-large { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor." }
          }
          div.("uk-width-auto@m") {
            a.uk-button.uk-button-default.uk-button-large href="#" { "Button" }
          }
        }
      }
    }
    div.uk-section {
      div.uk-container {
        div.uk-grid-large.uk-flex-middle data-uk-grid? {
          div.("uk-width-expand@m") {
            p.uk-text-large { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor." }
          }
          div.("uk-width-auto@m") {
            a.uk-button.uk-button-default.uk-button-large href="#" { "Button" }
          }
        }
      }
    }
  }
}

fn modals() -> Markup {
  html! {
    div#modal data-uk-modal? {
      div.uk-modal-dialog {
        button.uk-modal-close-default type="button" data-uk-close? {}
        div.uk-modal-header {
          h2.uk-modal-title { "Headline" }
        }
        div.uk-modal-body {
          p {
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
          }
        }
        div.uk-modal-footer.uk-text-right {
          button.uk-button.uk-button-default.uk-modal-close type="button" { "Cancel" }
          button.uk-button.uk-button-primary type="button" { "Save" }
        }
      }
    }
    div#modal-search.uk-modal-full data-uk-modal? {
      div.uk-modal-dialog.uk-flex.uk-flex-center.uk-flex-middle data-uk-height-viewport="" {
        button.uk-modal-close-full.uk-close-large type="button" data-uk-close? {}
        div {
          ul.uk-nav-primary.uk-nav-center data-uk-nav? {
            li.uk-active {
              a href="#" { "Active" }
            }
            li {
              a href="#" { "Item" }
            }
            li {
              a href="#" { "Item" }
            }
            li {
              a href="#" { "Item" }
            }
          }
          div.uk-margin {
            form.uk-search.uk-search-large {
              input.uk-search-input.uk-text-center type="search" placeholder="Search..." {}
            }
          }
        }
      }
    }
  }
}

fn offcanvas() -> Markup {
  html! {
    #offcanvas data-uk-offcanvas="flip: true; overlay: true" {
      div.uk-offcanvas-bar {
        button.uk-offcanvas-close type="button" data-uk-close? {}
        ul.uk-nav.uk-nav-default {
          li.uk-active {
            a href="#" { "Active" }
          }
          li.uk-parent {
            a href="#" { "Parent" }
            ul.uk-nav-sub {
              li {
                a href="#" { "Sub item" }
              }
              li {
                a href="#" { "Sub item" }
              }
            }
          }
          li.uk-nav-header { "Header" }
          li {
            a href="#" {
              span.uk-margin-small-right data-uk-icon="icon: table" {}
              " Item"
            }
          }
          li {
            a href="#" {
              span.uk-margin-small-right data-uk-icon="icon: thumbnails" {}
              " Item"
            }
          }
          li.uk-nav-divider {}
          li {
            a href="#" {
              span.uk-margin-small-right data-uk-icon="icon: trash" {}
              " Item"
            }
          }
        }
        h3 { "Title" }
        p {
          "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
        }
      }
    }
  }
}

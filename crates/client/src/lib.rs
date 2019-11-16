#![feature(proc_macro_hygiene)]
#![feature(exclusive_range_pattern)]
#![warn(anonymous_parameters)]
#![warn(bare_trait_objects)]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_debug_implementations)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/{{project-name}}/{{project-name}}/master/crates/assets/embed/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/{{project-name}}/{{project-name}}/master/crates/assets/embed/favicon.png")]
#![doc(issue_tracker_base_url = "https://github.com/{{project-name}}/{{project-name}}/issues/")]

//! `{{project-name}}-client` is run in the client's browser as a WebAssembly package.

#[macro_use]
pub(crate) mod logging;
mod ctx;
mod event_handler;
mod html;
mod js;
mod message_handler;
pub(crate) mod socket {
  pub(crate) mod ws;
  pub(crate) mod ws_events;
  pub(crate) mod ws_handlers;
}
pub(crate) mod templates {
  pub(crate) mod card;
}

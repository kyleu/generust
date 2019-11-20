#![feature(proc_macro_hygiene)]
#![warn(anonymous_parameters)]
#![warn(bare_trait_objects)]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_debug_implementations)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(unreachable_pub)]
#![forbid(unsafe_code)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![feature(exclusive_range_pattern)]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/{{project-name}}/{{project-name}}/master/crates/assets/embed/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/{{project-name}}/{{project-name}}/master/crates/assets/embed/favicon.png")]
#![doc(issue_tracker_base_url = "https://github.com/{{project-name}}/{{project-name}}/issues/")]

//! `{{project-name}}-templates` contains server HTML templates.

pub mod admin;
pub mod connections;
pub(crate) mod components {
  pub(crate) mod backtrace;
  pub(crate) mod card;
  pub(crate) mod colors;
  pub(crate) mod flash;
  pub(crate) mod header;
  pub(crate) mod navbar;
  pub(crate) mod page;
  pub(crate) mod script;
  pub(crate) mod simple;
}
pub mod error;
pub mod gallery;
pub mod home;
pub mod profile;
pub mod settings;
pub mod testbed;
pub(crate) mod utils;

pub(crate) use components::card::card;
pub(crate) use components::simple::{section, simple};

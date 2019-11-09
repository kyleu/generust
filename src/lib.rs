#![feature(option_expect_none)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]
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
#![warn(missing_docs)]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/{{project-name}}/{{project-name}}/master/crates/assets/embed/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/{{project-name}}/{{project-name}}/master/crates/assets/embed/favicon.png")]
#![doc(issue_tracker_base_url = "https://github.com/{{project-name}}/{{project-name}}/issues/")]
#![windows_subsystem = "windows"]

//! `{{project-name}}` is a work in progress. Docs soon.
//! - [{{project-name}}-assets]({{crate_name}}_assets)
//! - [{{project-name}}-client]({{crate_name}}_client)
//! - [{{project-name}}-controllers]({{crate_name}}_controllers)
//! - [{{project-name}}-core]({{crate_name}}_core)
//! - [{{project-name}}-service]({{crate_name}}_service)
//! - [{{project-name}}-templates]({{crate_name}}_templates)

mod app;
mod args;
mod cfg;
mod log;
mod server;

#[cfg(test)]
pub mod tests;

/// Application entrypoint, creates and starts the server
pub fn go() -> {{crate_name}}_core::Result<()> {
  let cfg = crate::cfg::cfg_from_args();
  crate::app::start(cfg)
}

/// External app entrypoint, calls `go()` directly and swallows errors
#[no_mangle]
pub extern "C" fn libgo() {
  match go() {
    Ok(_) => println!("Successfully started [{}]", {{crate_name}}_core::APPNAME),
    Err(e) => println!("Error starting [{}]: {}", {{crate_name}}_core::APPNAME, e)
  };
}

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
  extern crate jni;

  use self::jni::objects::JClass;
  use self::jni::JNIEnv;
  use super::go;

  #[no_mangle]
  #[allow(unsafe_code)]
  pub unsafe extern "C" fn Java_com_{{project-name}}_{{crate_name}}_{{crate_name}}_go(env: JNIEnv<'_>, _: JClass<'_>) {
    println!("Android!");
    go();
  }
}

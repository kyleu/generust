use actix_web::web::{get, post, resource};

/// Creates reverse routing table for [actix-web](actix-web)
pub fn add_routes(s: &mut actix_web::web::ServiceConfig) {
  let _ = s
    // Home
    .service(resource("/").name("home").route(get().to(crate::home::index)))
    .service(resource("/health").name("health").route(get().to(crate::home::health)))
    .service(resource("/profile").name("profile").route(get().to(crate::home::profile)).route(post().to(crate::home::profile_post)))

    // Testbed
    .service(resource("/testbed/{key}").name("testbed.detail").route(get().to(crate::testbed::testbed_key)))

    // Channels
    .service(resource("/s/{key}/connect").name("connect").route(get().to(crate::websocket::connect)))

    // Admin
    .service(resource("/admin").name("admin.list").route(get().to(crate::admin::list)))
    .service(resource("/admin/settings").name("admin.settings").route(get().to(crate::admin::settings)).route(post().to(crate::admin::settings_post)))
    .service(resource("/admin/conn").name("admin.connections").route(get().to(crate::admin::connections)))
    .service(resource("/admin/conn/{id}").name("admin.connection_detail").route(get().to(crate::admin::connection_detail)).route(post().to(crate::admin::connection_send)))
    .service(resource("/admin/channel/{id}").name("admin.channel_detail").route(get().to(crate::admin::channel_detail)).route(post().to(crate::admin::channel_send)))

    // Static
    .service(resource("/favicon.ico").name("favicon").route(get().to(crate::static_file::favicon)))
    .service(resource("/static/{filename:.*}").name("static").route(get().to(crate::static_file::static_file)))

    // Catch-all
    .service(resource("/{path:.*}").name("missing").to(crate::not_found));
}

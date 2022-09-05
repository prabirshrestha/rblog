// use crate::{appstate::AppState, handlers, routes};
use salvo::prelude::*;
// use trillium::{Handler, State};
// use trillium_caching_headers::caching_headers;
// use trillium_compression::compression;
// use trillium_conn_id::ConnId;
// use trillium_logger::{apache_combined, Logger};
// use trillium_router::Router;

#[handler]
async fn hello_world(res: &mut Response) {
    res.render("hello world");
}

pub fn app() -> Service {
    let router = Router::new().get(hello_world);
    Service::new(router)
}

// pub fn trillium_app() -> impl Handler {
//     (
//         State::new(AppState::new_from_env().unwrap()),
//         handlers::remove_server_response_header,
//         ConnId::new(),
//         Logger::new().with_formatter(apache_combined(
//             trillium_conn_id::log_formatter::conn_id,
//             "-",
//         )),
//         caching_headers(),
//         compression(),
//         Router::new()
//             .get("/", routes::posts::get_posts)
//             .get(
//                 "/posts/:slug",
//                 (handlers::ensure_trailing_slash, routes::posts::get_post),
//             )
//             .get("/posts/:slug/:attachment", routes::posts::get_attachment)
//             .get("/rss", routes::rss::get_rss_feed)
//             .get("/static/:name", routes::get_static_file)
//             .get("/healthcheck", routes::health_check)
//             .get("/robots.txt", routes::robots_txt),
//         routes::not_found,
//     )
// }

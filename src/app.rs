use crate::{appstate::AppState, handlers, routes};
use trillium::{Handler, Init, State};
use trillium_conn_id::ConnId;
use trillium_logger::{apache_combined, Logger};
use trillium_router::Router;

pub fn app() -> impl Handler {
    (
        Init::new(|_| async move {
            let state = AppState::new_from_env().unwrap();
            State::new(state)
        }),
        ConnId::new(),
        Logger::new().with_formatter(apache_combined("-", "-")),
        Router::new()
            .get("/", routes::posts::get_posts)
            .get(
                "/posts/:slug",
                (handlers::ensure_trailing_slash, routes::posts::get_post),
            )
            .get("/posts/:slug/:attachment", routes::posts::get_attachment)
            .get("/rss", routes::rss::get_rss_feed)
            .get("/static/:name", routes::get_static_file)
            .get("/healthcheck", routes::health_check),
        routes::not_found,
    )
}

use crate::{appstate::AppState, templates};
use trillium::{Conn, KnownHeaderName};
use trillium_ructe::RucteConnExt;

pub async fn get_rss_feed(conn: Conn) -> Conn {
    let state = conn.state::<AppState>().unwrap().to_owned();

    let blog = state.get_blog();
    let posts = blog
        .get_all_posts()
        .map(|key| state.get_blog().get_post(key).unwrap())
        .collect();

    conn.render(|o| templates::rss(o, blog, posts)).with_header(
        KnownHeaderName::ContentType,
        "application/rss+xml; charset=utf-8",
    )
}

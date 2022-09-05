pub mod posts;
pub mod rss;

use crate::templates::{self, statics::StaticFile};
use salvo::prelude::*;
use trillium::{conn_unwrap, Conn, KnownHeaderName, Status};
use trillium_router::RouterConnExt;
use trillium_ructe::RucteConnExt;

#[handler]
pub async fn health_check(res: &mut Response) {
    res.render(Text::Plain("OK"))
}

#[handler]
pub async fn robots_txt(res: &mut Response) {
    res.render(Text::Plain(
        r#"
User-agent: *

Disallow: /healthcheck
"#,
    ))
}

pub async fn not_found(conn: Conn) -> Conn {
    conn.render_html(|o| templates::notfound(o))
        .with_status(Status::NotFound)
}

pub async fn get_static_file(conn: Conn) -> Conn {
    let name = conn_unwrap!(conn.param("name"), conn);
    let data = conn_unwrap!(StaticFile::get(name), conn);
    conn.with_header(KnownHeaderName::ContentType, data.mime.to_string())
        .with_header(KnownHeaderName::CacheControl, "max-age=31536000") // 1 year as second
        .ok(data.content)
}

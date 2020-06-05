pub mod archives;
pub mod posts;
pub mod rss;

use crate::{appstate::AppState, templates};
use tide::{Request, Response, StatusCode};

pub async fn not_found(_ctx: Request<AppState>) -> tide::Result {
    let mut buf = Vec::new();
    templates::statuscode404(&mut buf)?;

    Ok(Response::new(StatusCode::NotFound)
        .body_string(String::from_utf8(buf)?)
        .set_mime(mime::TEXT_HTML_UTF_8))
}

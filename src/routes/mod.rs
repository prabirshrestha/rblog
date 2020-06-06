pub mod archives;
pub mod posts;
pub mod rss;

use crate::{appstate::AppState, templates};
use tide::{http::mime, Request, Response, StatusCode};

pub async fn not_found(_ctx: Request<AppState>) -> tide::Result {
    let mut buf = Vec::new();
    templates::statuscode404(&mut buf)?;

    let mut res = Response::new(StatusCode::NotFound);
    res.set_body(buf);
    res.set_content_type(mime::HTML);
    Ok(res)
}

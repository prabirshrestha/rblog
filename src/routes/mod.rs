pub mod archives;
pub mod posts;
pub mod rss;

use crate::{appstate::AppState, renderer::Render, templates};
use tide::{http::mime, Request, Response, StatusCode};

pub async fn not_found(_ctx: Request<AppState>) -> tide::Result {
    let mut res = Response::new(StatusCode::NotFound);
    res.render(|o| Ok(templates::notfound(o)?))?;
    res.set_content_type(mime::HTML);
    Ok(res)
}

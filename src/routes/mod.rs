pub mod archives;
pub mod posts;
pub mod rss;

use crate::{appstate::AppState, renderer::Render, templates};
use tide::{Request, Response, StatusCode};

pub async fn not_found(_req: Request<AppState>) -> tide::Result {
    let mut res = Response::new(StatusCode::NotFound);
    res.render_html(|o| Ok(templates::notfound(o)?))?;
    Ok(res)
}

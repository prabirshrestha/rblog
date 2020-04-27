pub mod archives;
pub mod posts;
pub mod rss;

use crate::appstate::AppState;
use tide::{Request, Response, StatusCode};

pub async fn not_found(_ctx: Request<AppState>) -> tide::Result {
    Ok(Response::new(StatusCode::NotFound).body_string("not found".to_owned()))
}

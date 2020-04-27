use crate::appstate::AppState;
use tide::{Request, Response, StatusCode};

pub async fn get_rss_feed(_ctx: Request<AppState>) -> tide::Result {
    return Ok(Response::new(StatusCode::Ok).body_string("hello world".to_owned()));
}

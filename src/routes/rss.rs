use crate::appstate::AppState;
use tide::{Request, Response, StatusCode};

pub async fn get_rss_feed(_req: Request<AppState>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body("rss feed");
    Ok(res)
}

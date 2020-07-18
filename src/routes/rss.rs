use crate::{appstate::AppState, renderer::RenderBuilder, templates};
use std::str::FromStr;
use tide::{http::Mime, Request, Response, StatusCode};

pub async fn get_rss_feed(req: Request<AppState>) -> tide::Result {
    let state = &req.state();

    let blog = state.get_blog();

    let posts = blog
        .get_all_posts()
        .map(|key| state.get_blog().get_post(key).unwrap())
        .collect();

    let res = Response::builder(StatusCode::Ok)
        .content_type(Mime::from_str("application/rss+xml; charset=utf-8")?)
        .render(|o| Ok(templates::rss(o, blog, posts)?))?
        .build();
    Ok(res)
}

use crate::{appstate::AppState, renderer::Render, templates};
use std::str::FromStr;
use tide::{http::Mime, Request, Response, StatusCode};

pub async fn get_rss_feed(req: Request<AppState>) -> tide::Result {
    let state = &req.state();

    let blog = state.get_blog();

    let posts = blog
        .get_all_posts()
        .map(|key| state.get_blog().get_post(key).unwrap())
        .collect();

    let mut res = Response::new(StatusCode::Ok);
    res.render_html(|o| Ok(templates::rss(o, blog, posts)?))?;
    res.set_content_type(Mime::from_str("application/rss+xml; charset=utf-8")?);
    Ok(res)
}

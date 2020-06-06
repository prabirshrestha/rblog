use crate::{appstate::AppState, renderer::Render, templates};
use tide::{Request, Response, StatusCode};

pub async fn get_archives(req: Request<AppState>) -> tide::Result {
    let state = &req.state();

    let posts = state
        .get_blog()
        .get_all_posts()
        .map(|key| state.get_blog().get_post(key).unwrap())
        .collect();

    let mut res = Response::new(StatusCode::Ok);
    res.render_html(|o| Ok(templates::archives(o, posts)?))?;

    Ok(res)
}

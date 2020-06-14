use crate::{appstate::AppState, renderer::Render, templates};
use tide::{http::mime, Request, Response, StatusCode};

pub async fn get_posts(req: Request<AppState>) -> tide::Result {
    let state = &req.state();

    let posts = state
        .get_blog()
        .get_all_posts()
        .map(|key| state.get_blog().get_post(key).unwrap())
        .collect();

    let mut res = Response::new(StatusCode::Ok);
    res.render_html(|o| Ok(templates::posts(o, posts)?))?;
    res.set_content_type(mime::HTML);

    Ok(res)
}

pub async fn get_post(ctx: Request<AppState>) -> tide::Result {
    let slug = ctx.param::<String>("slug")?;

    if let Some(post) = ctx.state().get_blog().get_post(&slug) {
        let mut res = Response::new(StatusCode::Ok);
        res.render_html(|o| Ok(templates::post(o, post)?))?;
        res.set_content_type(mime::HTML);
        return Ok(res);
    }

    let mut res = Response::new(StatusCode::NotFound);
    res.render_html(|o| Ok(templates::notfound(o)?))?;
    res.set_content_type(mime::HTML);
    Ok(res)
}

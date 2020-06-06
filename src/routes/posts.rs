use crate::{appstate::AppState, renderer::Render, templates};
use itertools::Itertools;
use tide::{http::mime, Request, Response, StatusCode};

pub async fn get_posts(req: Request<AppState>) -> tide::Result {
    let state = &req.state();

    let page = 1;

    let html = state
        .get_blog()
        .get_paged_posts(page)
        .map(|key| state.get_blog().get_post(key).unwrap())
        .map(|post| {
            String::from(format!(
                r#"<article><a href="{post_url}"><h2>{title}</h2></a>{content}</article>"#,
                post_url = post.get_url(),
                title = post.get_metadata().get_title(),
                content = post.get_content()
            ))
        })
        .join("");

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(html);
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

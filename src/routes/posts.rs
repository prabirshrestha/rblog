use crate::appstate::AppState;
use itertools::Itertools;
use tide::{Request, Response, StatusCode};

pub async fn get_posts(ctx: Request<AppState>) -> tide::Result {
    let state = &ctx.state();

    let page = 1;

    let body = state
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

    let res = Response::new(StatusCode::Ok)
        .body_string(body)
        .set_header("content-type".parse().unwrap(), "text/html;charset=utf-8");

    Ok(res)
}

pub async fn get_post(ctx: Request<AppState>) -> tide::Result {
    let slug = ctx.param::<String>("slug")?;

    if let Some(post) = ctx.state().get_blog().get_post(&slug) {
        return Ok(Response::new(StatusCode::Ok).body_string(post.get_content().to_owned()));
    }

    Ok(Response::new(StatusCode::NotFound).body_string("not found".to_owned()))
}

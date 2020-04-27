use crate::appstate::AppState;
use tide::{Request, Response, StatusCode};

pub async fn get_posts(_ctx: Request<AppState>) -> tide::Result {
    return Ok(Response::new(StatusCode::Ok).body_string("hello world".to_owned()));
}

pub async fn get_post(ctx: Request<AppState>) -> tide::Result {
    let slug = ctx.param::<String>("slug")?;

    if let Some(post) = ctx.state().get_blog().get_post(&slug) {
        return Ok(Response::new(StatusCode::Ok).body_string(post.get_content().to_owned()));
    }

    Ok(Response::new(StatusCode::NotFound).body_string("not found".to_owned()))
}

use crate::appstate::AppState;
use crate::blog::Post;
use tide::{Request, Response, StatusCode};

pub async fn get_archives(ctx: Request<AppState>) -> tide::Result {
    let state = &ctx.state();

    let posts: Vec<&Post> = state
        .get_blog()
        .get_all_posts()
        .map(|key| state.get_blog().get_post(key).unwrap())
        .collect();

    let mut body = String::from("");
    for post in posts {
        body.push_str(format!("{}<br/>", post.get_metadata().get_title()).as_str());
    }

    let res = Response::new(StatusCode::Ok)
        .body_string(body)
        .set_header("content-type".parse().unwrap(), "text/html;charset=utf-8");

    Ok(res)
}

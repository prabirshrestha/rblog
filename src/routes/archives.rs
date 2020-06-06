use crate::appstate::AppState;
use itertools::Itertools;
use tide::{http::mime, Request, Response, StatusCode};

pub async fn get_archives(req: Request<AppState>) -> tide::Result {
    let state = &req.state();

    let html = state
        .get_blog()
        .get_all_posts()
        .map(|key| state.get_blog().get_post(key).unwrap())
        .map(|post| {
            String::from(format!(
                r#"<a href="{href}"><h2>{title}</h2></a>"#,
                href = post.get_url(),
                title = post.get_metadata().get_title()
            ))
        })
        .join("");

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(html);
    res.set_content_type(mime::HTML);

    Ok(res)
}

use crate::appstate::AppState;
use itertools::Itertools;
use tide::{Request, Response, StatusCode};

pub async fn get_archives(ctx: Request<AppState>) -> tide::Result {
    let state = &ctx.state();

    let body = state
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

    let res = Response::new(StatusCode::Ok)
        .body_string(body)
        .set_mime(mime::TEXT_HTML_UTF_8);

    Ok(res)
}

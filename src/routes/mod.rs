pub mod posts;
pub mod rss;

use crate::{
    appstate::AppState, renderer::RenderBuilder, templates, templates::statics::StaticFile,
};
use tide::{Request, Response, StatusCode};

pub async fn health_check(_req: Request<AppState>) -> tide::Result {
    let res = Response::builder(StatusCode::Ok).build();
    Ok(res)
}

pub async fn not_found(_req: Request<AppState>) -> tide::Result {
    let res = Response::builder(StatusCode::NotFound)
        .render_html(|o| Ok(templates::notfound(o)?))?
        .build();
    Ok(res)
}

pub async fn get_static_file(req: Request<AppState>) -> tide::Result {
    let name = req.param("name")?;
    if let Some(data) = StaticFile::get(&name) {
        let res = Response::builder(StatusCode::Ok)
            .content_type(data.mime.clone())
            .header("cache-control", "max-age=31536000") // 1 year as second
            .body(data.content)
            .build();
        Ok(res)
    } else {
        let res = Response::new(StatusCode::NotFound);
        Ok(res)
    }
}

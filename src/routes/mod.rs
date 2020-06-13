pub mod archives;
pub mod posts;
pub mod rss;

use crate::{appstate::AppState, renderer::Render, templates, templates::statics::StaticFile};
use std::str::FromStr;
use tide::{http::Mime, Request, Response, StatusCode};

pub async fn not_found(_req: Request<AppState>) -> tide::Result {
    let mut res = Response::new(StatusCode::NotFound);
    res.render_html(|o| Ok(templates::notfound(o)?))?;
    Ok(res)
}

pub async fn get_static_file(req: Request<AppState>) -> tide::Result {
    let name = req.param::<String>("name")?;
    println!("{}", name);
    if let Some(data) = StaticFile::get(&name) {
        let mut res = Response::new(StatusCode::Ok);
        // TODO: add expires header
        res.set_content_type(Mime::from_str(data.mime.as_ref())?);
        res.set_body(data.content);
        Ok(res)
    } else {
        let mut res = Response::new(StatusCode::NotFound);
        res.set_body(name);
        Ok(res)
    }
}

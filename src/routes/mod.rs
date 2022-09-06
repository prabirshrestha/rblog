pub mod posts;
pub mod rss;

use crate::templates::statics::StaticFile;
use anyhow::{Context, Result};
use hyper::header::HeaderValue;
use salvo::{http::response::Body, prelude::*};

#[handler]
pub async fn health_check(res: &mut Response) {
    res.render(Text::Plain("OK"))
}

#[handler]
pub async fn robots_txt(res: &mut Response) {
    res.render(Text::Plain(
        r#"
User-agent: *

Disallow: /healthcheck
"#,
    ))
}

#[handler]
pub async fn get_static_file(req: &mut Request, res: &mut Response) -> Result<()> {
    let name = req.param("name").context("name not found")?;
    let data = StaticFile::get(name).context("Static File not found")?;
    let mime = data.mime.to_string();
    res.headers_mut()
        .insert("content-type", HeaderValue::from_str(&mime)?);
    res.headers_mut().insert(
        "cache-control",
        HeaderValue::from_static("max-age=31536000"),
    ); // 1 year as second
    res.set_body(Body::Once(data.content.into()));
    Ok(())
}

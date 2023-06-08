pub mod posts;
pub mod rss;

use crate::templates::statics::StaticFile;
use anyhow::{Context, Result};
use salvo::{http::header, prelude::*};

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
    res.add_header(header::CONTENT_TYPE, &data.mime.to_string(), true)?
        .add_header(
            header::CACHE_CONTROL,
            "max-age=31536000", // 1 year as second
            true,
        )?
        .write_body(data.content)?;
    Ok(())
}

use anyhow::{Context, Result};
pub use salvo::prelude::*;

use crate::templates::statics::StaticFile;

pub fn routes() -> Router {
    Router::with_path("/assets/<name>").get(get_assets)
}

#[handler]
fn get_assets(req: &mut Request, res: &mut Response) -> Result<()> {
    let name = req.param("name").context("No name parameter")?;
    let data = StaticFile::get(name).context("Static File not found")?;
    res.add_header(
        salvo::http::header::CONTENT_TYPE,
        &data.mime.to_string(),
        true,
    )?
    .add_header(
        salvo::http::header::CACHE_CONTROL,
        "max-age=31536000", // 1 year as second
        true,
    )?
    .write_body(data.content)?;
    Ok(())
}

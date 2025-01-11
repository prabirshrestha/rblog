use crate::{templates, utils::render::RenderExt};
use anyhow::Result;
use salvo::prelude::*;

pub fn routes() -> Router {
    Router::new().get(home)
}

#[handler]
async fn home(req: &mut Request, res: &mut Response) -> Result<()> {
    res.render_html(|o| templates::home::home_html(o))?;
    Ok(())
}

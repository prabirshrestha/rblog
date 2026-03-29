use crate::{app::AppDepot, templates, utils::render::RenderExt};
use anyhow::Result;
use salvo::prelude::*;

pub fn routes() -> Router {
    Router::new().get(get_home)
}

#[handler]
async fn get_home(res: &mut Response, depot: &mut Depot) -> Result<()> {
    let state = depot.app_state();

    let posts = state
        .blog_service
        .get_all_posts()
        .map(|key| state.blog_service.get_post(key).unwrap())
        .collect();

    res.render_html(|o| templates::home::home_html(o, &state.app_config, &posts))?;

    Ok(())
}

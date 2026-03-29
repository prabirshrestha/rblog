use crate::{
    app::AppDepot,
    templates,
    utils::render::RenderExt,
};
use anyhow::Result;
use salvo::prelude::*;

pub fn routes() -> Router {
    Router::new().push(Router::with_path("/rss").get(get_rss))
}

#[handler]
async fn get_rss(res: &mut Response, depot: &mut Depot) -> Result<()> {
    let state = depot.app_state();

    let posts = state.blog_service
        .get_all_posts()
        .map(|key| state.blog_service.get_post(key).unwrap())
        .collect();

    res.headers_mut()
        .insert("content-type", "application/rss+xml".parse().unwrap());
    res.render_template(|o| templates::rss::rss_html(o, &state.app_config, &posts))?;

    Ok(())
}

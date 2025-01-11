use crate::{
    app::{App, AppDepot},
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
    let App {
        blog_service,
        app_config,
    } = depot.app();

    let posts = blog_service
        .get_all_posts()
        .map(|key| blog_service.get_post(key).unwrap())
        .collect();

    res.render_html(|o| templates::rss::rss_html(o, app_config, &posts))?;

    Ok(())
}

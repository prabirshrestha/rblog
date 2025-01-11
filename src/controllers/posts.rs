use crate::{
    app::{App, AppDepot},
    templates,
    utils::render::RenderExt,
};
use anyhow::Result;
use salvo::prelude::*;

pub fn routes() -> Router {
    Router::with_path("/posts/<slug>").get(get_post)
}

#[handler]
async fn get_post(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<()> {
    let slug: &str = req.param("slug").unwrap_or_default();
    let normalized_slug = slug.to_lowercase();
    if slug != normalized_slug {
        res.render(Redirect::permanent(format!("/posts/{}/", normalized_slug)));
        return Ok(());
    }

    let App {
        blog_service,
        app_config,
    } = depot.app();

    if let Some(post) = blog_service.get_post(&normalized_slug) {
        res.render_html(|o| templates::posts::post_html(o, app_config, &post))?;
    } else {
        res.status_code(StatusCode::NOT_FOUND);
    }

    Ok(())
}

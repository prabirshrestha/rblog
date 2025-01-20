use crate::{
    app::{App, AppDepot},
    templates,
    utils::render::RenderExt,
};
use anyhow::Result;
use salvo::{fs::NamedFile, prelude::*};

pub fn routes() -> Router {
    Router::new()
        .push(Router::with_path("/posts/{slug}").get(get_post))
        .push(Router::with_path("/posts/{slug}/{attachment}").get(get_attachment))
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
        res.render_html(|o| templates::posts::post_html(o, app_config, post))?;
    } else {
        res.status_code(StatusCode::NOT_FOUND);
    }

    Ok(())
}

#[handler]
pub async fn get_attachment(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<()> {
    let slug: &str = req.param("slug").unwrap_or_default();
    let attachment_name: &str = req.param("attachment").unwrap_or_default();
    let normalized_slug = slug.to_lowercase();
    if slug != normalized_slug {
        res.render(Redirect::other(format!(
            "/posts/{}/{}",
            normalized_slug, attachment_name
        )));
        return Ok(());
    }

    let App { blog_service, .. } = depot.app();

    if let Some(post) = blog_service.get_post(slug) {
        if let Some(attachment) = post.attachments.get(attachment_name) {
            let file = NamedFile::open(&attachment.path).await?;
            file.send(req.headers(), res).await;
            return Ok(());
        }
    }

    res.status_code(StatusCode::NOT_FOUND);

    Ok(())
}

use crate::{appstate::AppState, render_html, templates};
use anyhow::Result;
use salvo::{fs::NamedFile, prelude::*};

#[handler]
pub async fn get_posts(depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot.obtain::<AppState>().unwrap();

    let blog = state.get_blog();

    let posts = blog
        .get_all_posts()
        .map(|key| blog.get_post(key).unwrap())
        .collect();

    render_html(res, |o| templates::posts_html(o, blog, &posts))?;

    Ok(())
}

#[handler]
pub async fn get_post(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let slug: &str = req.param("slug").unwrap_or_default();
    let normalized_slug = slug.to_lowercase();
    if slug != normalized_slug {
        res.render(Redirect::permanent(format!("/posts/{}/", normalized_slug)));
        return Ok(());
    }

    let state = depot.obtain::<AppState>().unwrap();

    let blog = state.get_blog();
    if let Some(post) = blog.get_post(slug) {
        render_html(res, |o| templates::post_html(o, blog, post))?;
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

    let state = depot.obtain::<AppState>().unwrap();
    let blog = state.get_blog();

    if let Some(post) = blog.get_post(slug) {
        if let Some(attachment) = post.get_attachment(attachment_name) {
            let file = NamedFile::open(attachment.get_path()).await?;
            file.send(req.headers(), res).await;
            return Ok(());
        }
    }

    res.status_code(StatusCode::NOT_FOUND);

    Ok(())
}

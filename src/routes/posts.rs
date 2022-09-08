use crate::{appstate::AppState, templates};
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

    let mut buf = Vec::new();
    templates::posts_html(&mut buf, blog, posts)?;
    res.render(Text::Html(String::from_utf8(buf)?));

    Ok(())
}

#[handler]
pub async fn get_post(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let slug: &str = req.param("slug").unwrap_or_default();
    let normalized_slug = slug.to_lowercase();
    if slug != normalized_slug {
        res.render(Redirect::permanent(&format!("/posts/{}/", normalized_slug)));
        return Ok(());
    }

    let state = depot.obtain::<AppState>().unwrap();

    let blog = state.get_blog();
    if let Some(post) = blog.get_post(slug) {
        let mut buf = Vec::new();
        templates::post_html(&mut buf, blog, post)?;
        res.render(Text::Html(String::from_utf8(buf)?));
    } else {
        res.with_status_code(StatusCode::NOT_FOUND);
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
        res.render(Redirect::permanent(&format!(
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

    res.with_status_code(StatusCode::NOT_FOUND);
    Ok(())
}

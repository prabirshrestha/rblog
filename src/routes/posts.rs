use crate::blog::Post;
use crate::{appstate::AppState, templates};
use anyhow::Context;
use anyhow::Result;
use ructe::templates::ToHtml;
use salvo::prelude::*;
use trillium::{conn_unwrap, Conn, KnownHeaderName, Status};
use trillium_router::RouterConnExt;
use trillium_ructe::RucteConnExt;
use trillium_static::StaticConnExt;

#[handler]
pub async fn get_posts(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
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

pub async fn get_post(conn: Conn) -> Conn {
    let slug = conn.param("slug").unwrap_or_default();
    let normalized_slug = slug.to_lowercase();
    if slug != normalized_slug {
        return conn
            .with_status(Status::PermanentRedirect)
            .with_header(
                KnownHeaderName::Location,
                format!("/posts/{}/", normalized_slug),
            )
            .halt();
    }

    let state = conn.state::<AppState>().unwrap().to_owned();
    let blog = state.get_blog();

    let post = conn_unwrap!(blog.get_post(slug), conn);
    conn.render_html(|o| templates::post(o, blog, post))
}

pub async fn get_attachment(conn: Conn) -> Conn {
    let slug = conn.param("slug").unwrap_or_default();
    let attachment_name = conn_unwrap!(conn.param("attachment"), conn);
    let normalized_slug = slug.to_lowercase();
    if slug != normalized_slug {
        let attachment_name = attachment_name.to_owned();
        return conn
            .with_status(Status::PermanentRedirect)
            .with_header(
                KnownHeaderName::Location,
                format!("/posts/{}/{}", normalized_slug, attachment_name),
            )
            .halt();
    }

    let state = conn.state::<AppState>().unwrap().to_owned();
    let blog = state.get_blog();

    let post = conn_unwrap!(blog.get_post(slug), conn);
    let attachment = conn_unwrap!(post.get_attachment(attachment_name), conn);
    conn.send_path(attachment.get_path()).await
}

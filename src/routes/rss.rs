use crate::{appstate::AppState, templates};
use anyhow::Result;
use hyper::http::HeaderValue;
use salvo::prelude::*;

#[handler]
pub async fn rss_feed(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot.obtain::<AppState>().unwrap();

    let blog = state.get_blog();
    let posts = blog
        .get_all_posts()
        .map(|key| state.get_blog().get_post(key).unwrap())
        .collect();

    let mut buf = Vec::new();
    templates::rss(&mut buf, blog, posts)?;
    res.render(String::from_utf8(buf)?);

    res.headers_mut().insert(
        "Content-Type",
        HeaderValue::from_static("application/rss+xml; charset=utf-8"),
    );

    Ok(())
}

use crate::{appstate::AppState, templates};
use anyhow::Result;
use salvo::prelude::*;

#[handler]
pub async fn rss_feed(depot: &mut Depot, res: &mut Response) -> Result<()> {
    let state = depot.obtain::<AppState>().unwrap();

    let blog = state.get_blog();
    let posts = blog
        .get_all_posts()
        .map(|key| state.get_blog().get_post(key).unwrap())
        .collect();

    let mut buf = Vec::new();
    templates::rss(&mut buf, blog, posts)?;
    res.render(Text::Rss(String::from_utf8(buf)?));

    Ok(())
}

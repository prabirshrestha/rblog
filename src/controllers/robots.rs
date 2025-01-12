use crate::{
    app::{App, AppDepot},
    templates,
    utils::render::RenderExt,
};
use anyhow::Result;
use salvo::prelude::*;

pub fn routes() -> Router {
    Router::new().push(Router::with_path("/robots.txt").get(get_robots_txt))
}

#[handler]
async fn get_robots_txt(res: &mut Response, depot: &mut Depot) -> Result<()> {
    res.render(Text::Plain(
        r#"
User-agent: *
Disallow: /healthcheck
"#,
    ));

    Ok(())
}

use anyhow::Result;
use salvo::prelude::*;

pub fn routes() -> Router {
    Router::new().push(Router::with_path("/robots.txt").get(get_robots_txt))
}

#[handler]
fn get_robots_txt(res: &mut Response) -> Result<()> {
    res.render(Text::Plain(
        r#"
User-agent: *
Disallow: /healthcheck
"#,
    ));

    Ok(())
}

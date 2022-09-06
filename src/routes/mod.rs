pub mod posts;
pub mod rss;

use salvo::prelude::*;

#[handler]
pub async fn health_check(res: &mut Response) {
    res.render(Text::Plain("OK"))
}

#[handler]
pub async fn robots_txt(res: &mut Response) {
    res.render(Text::Plain(
        r#"
User-agent: *

Disallow: /healthcheck
"#,
    ))
}

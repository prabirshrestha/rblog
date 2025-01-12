use anyhow::Result;
use salvo::prelude::*;

pub fn routes() -> Router {
    Router::new().push(Router::with_path("/healthcheck").get(get_healthcheck))
}

#[handler]
fn get_healthcheck(res: &mut Response) -> Result<()> {
    res.render(Text::Plain("OK"));
    Ok(())
}

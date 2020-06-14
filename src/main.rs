mod appstate;
mod blog;
mod renderer;
mod routes;

use crate::appstate::AppState;
use anyhow::Result;
use tide::log;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

#[async_std::main]
async fn main() -> Result<()> {
    log::start();

    let state = AppState::new_from_env()?;
    let addr = state.get_addr()?;

    let mut app = tide::with_state(state);
    register_routes(&mut app);

    app.listen(addr).await?;

    Ok(())
}

fn register_routes(app: &mut tide::Server<AppState>) {
    app.at("/").get(routes::posts::get_posts);
    app.at("/posts/:slug").get(routes::posts::get_post);
    app.at("/rss").get(routes::rss::get_rss_feed);
    app.at("/static/*name").get(routes::get_static_file);
    app.at("*").all(routes::not_found);
}

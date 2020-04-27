mod appstate;
mod blog;
mod routes;

use crate::appstate::AppState;
use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    let state = AppState::new_from_env()?;
    let addr = state.get_addr()?;

    let mut app = tide::with_state(state);
    register_routes(&mut app);
    app.listen(addr).await?;

    Ok(())
}

fn register_routes(app: &mut tide::Server<AppState>) {
    app.at("/").get(routes::posts::get_posts);
    app.at("/page/:page").get(routes::posts::get_posts);
    app.at("/posts/:slug").get(routes::posts::get_post);
    app.at("/archives").get(routes::archives::get_archives);
    app.at("*").all(routes::not_found);
}

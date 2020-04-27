mod appstate;
mod blog;
mod routes;

use crate::appstate::AppState;
use anyhow::Result;
use tide::{Request, Response, StatusCode};

#[async_std::main]
async fn main() -> Result<()> {
    let state = AppState::new_from_env()?;
    let addr = state.get_addr().to_string();

    let mut app = tide::with_state(state);
    register_routes(&mut app);
    app.listen(addr).await?;

    Ok(())
}

fn register_routes(app: &mut tide::Server<AppState>) {
    app.at("/").get(|_| async { Ok("Hello world") });
    app.at("/posts/:slug").get(handle_get_post);
    app.at("/archives").get(routes::archives::get_archives);
    app.at("*").all(|_| async {
        Ok(Response::new(StatusCode::NotFound).body_string("not found".to_owned()))
    });
}

async fn handle_get_post(ctx: Request<AppState>) -> tide::Result {
    let slug = ctx.param::<String>("slug")?;

    if let Some(post) = ctx.state().get_blog().get_post(&slug) {
        return Ok(Response::new(StatusCode::Ok).body_string(post.get_content().to_owned()));
    }

    Ok(Response::new(StatusCode::NotFound).body_string("not found".to_owned()))
}

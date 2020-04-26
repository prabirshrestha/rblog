use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;
use tide::{Response, StatusCode};

#[derive(Debug)]
pub struct AppState {
    addr: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlogConf {
    title: String,
    page_size: Option<u16>,
    enable_drafts: Option<bool>,
}

impl AppState {
    pub fn new_from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()?;
        let addr = format!("0.0.0.0:{}", port);

        Ok(Self { addr })
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let state = AppState::new_from_env()?;
    let addr = state.addr.clone();

    let mut app = tide::with_state(state);
    app = register_routes(app);
    app.listen(addr).await?;

    Ok(())
}

fn register_routes(mut app: tide::Server<AppState>) -> tide::Server<AppState> {
    app.at("/").get(|_| async { Ok("Hello world") });
    app.at("*").all(|_| async {
        Ok(Response::new(StatusCode::NotFound).body_string(String::from("not found")))
    });
    app
}

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use tide::{Response, StatusCode};

#[derive(Debug)]
pub struct AppState {
    addr: String,
    conf: BlogConf,
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

        let args: Vec<String> = env::args().collect();
        let conf_path = if args.len() == 2 {
            Path::new(&args[1])
        } else {
            Path::new("./blog.conf")
        };

        if !conf_path.exists() {
            bail!("File not found - {:?}", conf_path);
        }

        let conf_contents = fs::read_to_string(conf_path)?;
        let conf: BlogConf = serde_yaml::from_str(&conf_contents)?;

        let port = env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()?;
        let addr = format!("0.0.0.0:{}", port);

        Ok(Self { addr, conf })
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

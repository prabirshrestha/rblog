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
    posts_dir: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    metadata: PostMetadata,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostMetadata {
    title: String,
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

        let conf = BlogConf::new_from_file(&conf_path)?;

        let port = env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()?;

        let addr = format!("0.0.0.0:{}", port);

        Ok(Self { addr, conf })
    }
}

impl BlogConf {
    pub fn new_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            bail!("File not found - {:?}", &path);
        }

        let conf_contents = fs::read_to_string(&path)?;
        let mut conf: BlogConf = serde_yaml::from_str(&conf_contents)?;

        if let None = &conf.posts_dir {
            conf.posts_dir = Some(String::from("./posts"));
        }

        if !Path::new(&conf.posts_dir.as_ref().unwrap()).exists() {
            bail!(
                "Directory not found - {:?}",
                conf.posts_dir.as_ref().unwrap()
            );
        }

        Ok(conf)
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

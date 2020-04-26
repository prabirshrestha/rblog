use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use tide::{Request, Response, StatusCode};

#[derive(Debug)]
pub struct AppState {
    addr: String,
    conf: BlogConf,
    posts: HashMap<String, Post>,
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
    slug: Option<String>,
    date: Option<DateTime<Utc>>,
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

        let posts = Post::read_all_from_dir(Path::new(conf.posts_dir.as_ref().unwrap()))?;

        Ok(Self { addr, conf, posts })
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

impl Post {
    pub fn read_all_from_dir(path: &Path) -> Result<HashMap<String, Post>> {
        let mut map = HashMap::new();

        let paths = fs::read_dir(path)?;
        for path in paths {
            let path = path?.path();
            let metadata = fs::metadata(&path)?;
            if metadata.is_file() {
                let post = Post::new_from_file(&path)?;
                let key = post.metadata.slug.as_ref().unwrap();
                if map.contains_key(key) {
                    bail!("Post {:?} already exists", &path);
                } else {
                    map.insert(String::from(key), post);
                }
            }
        }

        Ok(map)
    }

    pub fn new_from_str(raw: &str) -> Result<Post> {
        let header_start = raw.find("---");
        if header_start.is_none() {
            bail!("--- header not found");
        }
        let header_start = header_start.unwrap();

        let content_start = &raw[header_start + 3..].find("---");
        if content_start.is_none() {
            bail!("--- content not found");
        }
        let content_start = content_start.unwrap();

        let header = &raw[header_start..content_start + 3];
        let content = &raw[header.len() + 3..];

        let mut metadata: PostMetadata = serde_yaml::from_str(header)?;

        metadata.slug = match &metadata.slug {
            Some(slug) => Some(slug.trim().to_lowercase()),
            None => Some(slugify(&metadata.title)),
        };

        let post = Post {
            metadata,
            content: String::from(content.trim()),
        };

        Ok(post)
    }

    pub fn new_from_file(path: &Path) -> Result<Post> {
        let raw = fs::read_to_string(path)?;
        Post::new_from_str(&raw)
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
    app.at("/posts/:slug").get(handle_get_post);
    app.at("/archives").get(handle_get_archives);
    app.at("*").all(|_| async {
        Ok(Response::new(StatusCode::NotFound).body_string("not found".to_owned()))
    });
    app
}

async fn handle_get_post(ctx: Request<AppState>) -> tide::Result {
    let slug = ctx.param::<String>("slug")?;

    if let Some(post) = ctx.state().posts.get(&slug) {
        return Ok(Response::new(StatusCode::Ok).body_string(post.content.clone()));
    }

    Ok(Response::new(StatusCode::NotFound).body_string("not found".to_owned()))
}

async fn handle_get_archives(_ctx: Request<AppState>) -> tide::Result {
    Ok(Response::new(StatusCode::Found).body_string("archives".to_owned()))
}

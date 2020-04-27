mod blog;

use crate::blog::{Blog, BlogConf, Post};
use anyhow::Result;
use std::env;
use std::path::Path;
use tide::{Request, Response, StatusCode};

#[derive(Debug)]
pub struct AppState {
    addr: String,
    blog: Blog,
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

        Ok(Self {
            addr,
            blog: Blog::from_conf(conf)?,
        })
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

    if let Some(post) = ctx.state().blog.get_post(&slug) {
        return Ok(Response::new(StatusCode::Ok).body_string(post.get_content().to_owned()));
    }

    Ok(Response::new(StatusCode::NotFound).body_string("not found".to_owned()))
}

async fn handle_get_archives(ctx: Request<AppState>) -> tide::Result {
    let state = &ctx.state();

    let ordered_posts = &state.blog.get_all_posts();

    let posts: Vec<&Post> = ordered_posts
        .into_iter()
        .map(|key| state.blog.get_post(key).unwrap())
        .collect();

    let mut body = String::from("");
    for post in posts {
        body.push_str(format!("{}<br/>", post.get_metadata().get_title()).as_str());
    }

    let res = Response::new(StatusCode::Ok)
        .body_string(body)
        .set_header("content-type".parse().unwrap(), "text/html;charset=utf-8");

    Ok(res)
}

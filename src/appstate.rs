use crate::blog::{Blog, BlogConf};
use anyhow::Result;
use std::env;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    blog: Arc<Blog>,
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

        let conf = BlogConf::new_from_file(conf_path)?;

        Ok(Self {
            blog: Arc::new(Blog::from_conf(conf)?),
        })
    }

    pub fn get_blog(&self) -> &Blog {
        &self.blog
    }
}

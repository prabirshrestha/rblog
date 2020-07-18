use crate::blog::{Blog, BlogConf};
use anyhow::Result;
use std::env;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    addr: String,
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

        let conf = BlogConf::new_from_file(&conf_path)?;

        let port = env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()?;

        let addr = format!("0.0.0.0:{}", port);

        Ok(Self {
            addr,
            blog: Arc::new(Blog::from_conf(conf)?),
        })
    }

    pub fn get_blog(&self) -> &Blog {
        &self.blog
    }

    pub fn get_addr(&self) -> Result<SocketAddr> {
        let addr: SocketAddr = self.addr.parse()?;
        Ok(addr)
    }
}

use crate::{appstate::AppState, render_html, routes, templates};
use anyhow::Result;
use listenfd::ListenFd;
use salvo::{extra, prelude::*, Catcher};
use std::net::SocketAddr;

pub async fn run() -> Result<()> {
    let mut listenfd = ListenFd::from_env();
    let (addr, server) = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        (
            listener.local_addr()?,
            hyper::server::Server::from_tcp(listener)?,
        )
    } else {
        let addr: SocketAddr = "127.0.0.1:8080".parse()?;
        (addr, hyper::server::Server::bind(&addr))
    };

    tracing::info!("Listening on {}", addr);

    server.serve(make_service().await?).await?;

    Ok(())
}

async fn make_service() -> Result<Service> {
    let router = Router::new()
        .hoop(extra::affix::inject(AppState::new_from_env()?))
        .hoop(extra::logging::Logger::default())
        .hoop(extra::compression::Compression::default().with_force_priority(true)) // Compression must be before CachingHeader.
        .hoop(extra::caching_headers::CachingHeaders::default())
        .get(routes::posts::get_posts)
        .push(
            Router::with_path("/posts/<slug>")
                .hoop(extra::trailing_slash::add_slash())
                .get(routes::posts::get_post),
        )
        .push(Router::with_path("/posts/<slug>/<attachment>").get(routes::posts::get_attachment))
        .push(Router::with_path("/static/<name>").get(routes::get_static_file))
        .push(Router::with_path("/rss").get(routes::rss::rss_feed))
        .push(Router::with_path("/healthcheck").get(routes::health_check))
        .push(Router::with_path("/robots.txt").get(routes::robots_txt));
    let catchers: Vec<Box<dyn Catcher>> = vec![Box::new(NotFoundCatcher)];
    Ok(Service::new(router).with_catchers(catchers))
}

struct NotFoundCatcher;

impl Catcher for NotFoundCatcher {
    fn catch(&self, _req: &Request, _depot: &Depot, res: &mut Response) -> bool {
        if let Some(StatusCode::NOT_FOUND) = res.status_code() {
            match render_html(res, |o| templates::notfound(o)) {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }
}

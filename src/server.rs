use std::net::SocketAddr;

use crate::{appstate::AppState, render_html, routes, templates};
use anyhow::Result;
use listenfd::ListenFd;
use salvo::{catcher::Catcher, conn::tcp::TcpAcceptor, prelude::*};

pub async fn run() -> Result<()> {
    let mut listenfd = ListenFd::from_env();
    let (addr, listener) = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        (
            listener.local_addr()?,
            tokio::net::TcpListener::from_std(listener).unwrap(),
        )
    } else {
        let addr: SocketAddr = format!(
            "{}:{}",
            std::env::var("HOST").unwrap_or("127.0.0.1".into()),
            std::env::var("PORT").unwrap_or("8080".into())
        )
        .parse()?;
        (addr, tokio::net::TcpListener::bind(addr).await.unwrap())
    };

    tracing::info!("Listening on {}", addr);
    let acceptor = TcpAcceptor::try_from(listener).unwrap();
    Server::new(acceptor).serve(make_service().await?).await;

    Ok(())
}

async fn make_service() -> Result<Service> {
    let router = Router::new()
        .hoop(salvo::affix::inject(AppState::new_from_env()?))
        .hoop(salvo::logging::Logger::default())
        // .hoop(salvo::caching_headers::CachingHeaders::default()) // CachingHeader must be before Compression.
        // .hoop(salvo::compression::Compression::default().with_force_priority(true))
        .get(routes::posts::get_posts)
        .push(
            Router::with_path("/posts/<slug>")
                .hoop(salvo::trailing_slash::add_slash())
                .get(routes::posts::get_post),
        )
        .push(Router::with_path("/posts/<slug>/<attachment>").get(routes::posts::get_attachment))
        .push(Router::with_path("/static/<name>").get(routes::get_static_file))
        .push(Router::with_path("/rss").get(routes::rss::rss_feed))
        .push(Router::with_path("/healthcheck").get(routes::health_check))
        .push(Router::with_path("/robots.txt").get(routes::robots_txt));
    Ok(Service::new(router).catcher(Catcher::default().hoop(not_found_catcher)))
}

#[handler]
async fn not_found_catcher(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
    ctrl: &mut FlowCtrl,
) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        match render_html(res, |o| templates::notfound_html(o)) {
            Ok(_) => {}
            Err(_) => {
                ctrl.call_next(req, depot, res).await;
            }
        }
    } else {
        ctrl.call_next(req, depot, res).await;
    }
}

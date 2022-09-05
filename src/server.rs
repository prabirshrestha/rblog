use anyhow::Result;
use listenfd::ListenFd;
use salvo::{extra::logging::LogHandler, prelude::*};
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

#[handler]
async fn hello_world(res: &mut Response) {
    res.render("hello world");
}

async fn make_service() -> Result<Service> {
    let router = Router::new().hoop(LogHandler).get(hello_world);
    Ok(Service::new(router))
}

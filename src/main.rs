use listenfd::ListenFd;
use salvo::extra::logging::LogHandler;
use salvo::prelude::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    // trillium_tokio::run(app());
    serve().await;
}

#[handler]
async fn hello_world(res: &mut Response) {
    res.render("hello world");
}

async fn serve() {
    let mut listenfd = ListenFd::from_env();
    let server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        hyper::server::Server::from_tcp(listener).unwrap()
    } else {
        let addr: SocketAddr = "127.0.0.1:8080"
            .parse()
            .expect("Unable to parse socket address");
        hyper::server::Server::bind(&addr)
    };

    tracing::info!("Listening on http://127.0.0.1:8080");

    let router = Router::new().hoop(LogHandler).get(hello_world);
    server.serve(Service::new(router)).await.unwrap();
}

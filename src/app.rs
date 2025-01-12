use std::sync::Arc;

use crate::{app_config::AppConfig, controllers, services::blog::BlogService};
use anyhow::Result;
use salvo::{catcher::Catcher, prelude::*, server::ServerHandle};
use tokio::signal;
use tracing::info;

#[derive(Clone)]
pub struct App {
    pub app_config: Arc<AppConfig>,
    pub blog_service: Arc<BlogService>,
}

impl App {
    pub fn version() -> String {
        format!(
            "{} {}-{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            &env!("VERGEN_GIT_SHA")[..9]
        )
    }

    pub async fn from_env() -> Result<Self> {
        let app_config = Arc::new(AppConfig::from_config_file("blog.yaml")?);
        Self::from_config(app_config).await
    }

    pub async fn from_config(app_config: Arc<AppConfig>) -> Result<Self> {
        let blog_service = Arc::new(BlogService::new(app_config.clone())?);
        let app = Self {
            app_config,
            blog_service,
        };

        Ok(app)
    }

    pub fn app_config(&self) -> &AppConfig {
        &self.app_config
    }

    pub async fn run(self) -> Result<()> {
        info!("Starting server");

        let acceptor = TcpListener::new(format!(
            "{}:{}",
            &self.app_config().host,
            &self.app_config().port
        ))
        .bind()
        .await;

        let server = Server::new(acceptor);
        let handle = server.handle();

        tokio::spawn(shutdown_signal(handle));

        let router = Router::new()
            .hoop(salvo::affix_state::inject(self.clone()))
            .hoop(Logger::new())
            .push(controllers::router());

        let service =
            Service::new(router).catcher(Catcher::default().hoop(controllers::errors::not_found));

        server.serve(service).await;

        Ok(())
    }
}

async fn shutdown_signal(handle: ServerHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("ctrl_c signal received"),
        _ = terminate => info!("terminate signal received"),
    }

    handle.stop_graceful(std::time::Duration::from_secs(60));
}

pub trait AppDepot {
    fn app(&self) -> &App;
}

impl AppDepot for Depot {
    fn app(&self) -> &App {
        self.obtain::<App>().unwrap()
    }
}

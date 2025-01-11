use std::sync::Arc;

use crate::{app_config::AppConfig, controllers};
use salvo::{prelude::*, server::ServerHandle};
use tokio::signal;
use tracing::info;

#[derive(Clone)]
pub struct App {
    pub app_config: Arc<AppConfig>,
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

    pub async fn new_from_env() -> anyhow::Result<Self> {
        let app_config = Arc::new(AppConfig::from_path()?);
        Self::new_from_config(app_config).await
    }

    pub async fn new_from_config(app_config: Arc<AppConfig>) -> anyhow::Result<Self> {
        let app = Self { app_config };

        Ok(app)
    }

    pub fn app_config(&self) -> &AppConfig {
        &self.app_config
    }

    pub async fn run(self) -> anyhow::Result<()> {
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

        let service = Router::new()
            .hoop(salvo::affix_state::inject(self.clone()))
            .push(controllers::router());

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

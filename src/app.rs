use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};

use arc_swap::ArcSwap;

use crate::{app_config::AppConfig, controllers, services::blog::BlogService};
use anyhow::Result;
use salvo::{catcher::Catcher, prelude::*, server::ServerHandle};
use tokio::signal;
use tracing::{info, warn};

pub struct AppState {
    pub app_config: Arc<AppConfig>,
    pub blog_service: Arc<BlogService>,
}

#[derive(Clone)]
pub struct App {
    pub config_file: String,
    pub state: Arc<ArcSwap<AppState>>,
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
        Self::from_config_file("blog.yaml").await
    }

    pub async fn from_config_file(config_file: &str) -> Result<Self> {
        let app_config = Arc::new(AppConfig::from_config_file(config_file)?);
        let blog_service = Arc::new(BlogService::new(app_config.clone())?);
        Ok(Self {
            config_file: config_file.to_string(),
            state: Arc::new(ArcSwap::new(Arc::new(AppState {
                app_config,
                blog_service,
            }))),
        })
    }

    pub async fn run(self) -> Result<()> {
        info!("Starting server");

        let (host, port, watch_interval) = {
            let s = self.state.load();
            (
                s.app_config.host.clone(),
                s.app_config.port.clone(),
                s.app_config.watch.interval,
            )
        };

        let acceptor = TcpListener::new(format!("{}:{}", host, port))
            .bind()
            .await;

        let server = Server::new(acceptor);
        let handle = server.handle();

        tokio::spawn(shutdown_signal(handle));
        tokio::spawn(watch_for_changes(
            self.config_file.clone(),
            self.state.clone(),
            watch_interval,
        ));

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

fn max_mtime_in_dir(path: &Path) -> std::io::Result<SystemTime> {
    let mut max = std::fs::metadata(path)?.modified()?;
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            continue;
        }
        let mtime = entry.metadata()?.modified()?;
        if mtime > max {
            max = mtime;
        }
        if file_type.is_dir() {
            if let Ok(sub_max) = max_mtime_in_dir(&entry.path()) {
                if sub_max > max {
                    max = sub_max;
                }
            }
        }
    }
    Ok(max)
}

fn dir_fingerprint(path: &Path) -> Option<(PathBuf, SystemTime)> {
    let canonical = path.canonicalize().ok()?;
    let mtime = max_mtime_in_dir(&canonical).ok()?;
    Some((canonical, mtime))
}

fn file_mtime(path: &Path) -> Option<SystemTime> {
    std::fs::metadata(path).ok()?.modified().ok()
}

async fn watch_for_changes(
    config_file: String,
    state: Arc<ArcSwap<AppState>>,
    interval_secs: u64,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
    interval.tick().await; // skip the immediate first tick

    let config_path = PathBuf::from(&config_file);

    let posts_dir = {
        let s = state.load();
        PathBuf::from(&s.app_config.posts_dir)
    };
    let mut last_posts_fp = dir_fingerprint(&posts_dir);
    let mut last_config_mtime = file_mtime(&config_path);

    loop {
        interval.tick().await;

        let (watch_enabled, posts_dir) = {
            let s = state.load();
            (s.app_config.watch.enabled, PathBuf::from(&s.app_config.posts_dir))
        };

        if !watch_enabled {
            info!("Watching disabled, stopping watcher");
            return;
        }

        let current_config_mtime = file_mtime(&config_path);
        let current_posts_fp = dir_fingerprint(&posts_dir);

        if current_posts_fp == last_posts_fp && current_config_mtime == last_config_mtime {
            continue;
        }

        info!("Change detected, reloading blog...");

        let new_config = match AppConfig::from_config_file(&config_file) {
            Ok(c) => Arc::new(c),
            Err(e) => {
                warn!("Failed to reload config: {}", e);
                continue;
            }
        };

        let new_posts_dir = PathBuf::from(&new_config.posts_dir);

        match BlogService::new(new_config.clone()) {
            Ok(new_service) => {
                state.store(Arc::new(AppState {
                    app_config: new_config,
                    blog_service: Arc::new(new_service),
                }));
                last_posts_fp = dir_fingerprint(&new_posts_dir);
                last_config_mtime = file_mtime(&config_path);
                info!("Blog reloaded successfully");
            }
            Err(e) => warn!("Failed to reload blog service: {}", e),
        }
    }
}

pub trait AppDepot {
    fn app(&self) -> &App;
}

impl AppDepot for Depot {
    fn app(&self) -> &App {
        self.obtain::<App>().unwrap()
    }
}

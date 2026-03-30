use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use arc_swap::ArcSwap;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

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

        let acceptor = TcpListener::new(format!("{}:{}", host, port)).bind().await;

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

/// Walk up every component of `path` and collect the parent directory of each
/// symlink found. These parents are watched non-recursively so that any
/// atomic symlink swap (e.g. git-sync renaming `current`) fires an event.
///
/// Returns `(watchable_parents, needs_fallback)` where `needs_fallback` is true
/// if any symlink's parent is the filesystem root (unwatchable on any platform).
fn symlink_parents(path: &Path) -> (Vec<PathBuf>, bool) {
    let mut parents = Vec::new();
    let mut needs_fallback = false;
    let mut current = path.to_path_buf();
    loop {
        if std::fs::symlink_metadata(&current)
            .map(|m| m.is_symlink())
            .unwrap_or(false)
        {
            if let Some(parent) = current.parent() {
                let parent = parent.to_path_buf();
                // parent.parent().is_none() is true for filesystem roots on all
                // platforms: `/` on Unix, `C:\` / `\\server\share` on Windows.
                if parent.parent().is_none() {
                    needs_fallback = true;
                } else if !parents.contains(&parent) {
                    parents.push(parent);
                }
            }
        }
        if !current.pop() {
            break;
        }
    }
    (parents, needs_fallback)
}

/// Resolves to `interval.tick()` if Some, or pends forever if None.
/// Used to conditionally include a fallback timer in `tokio::select!`.
async fn fallback_tick(interval: &mut Option<tokio::time::Interval>) {
    match interval.as_mut() {
        Some(i) => {
            i.tick().await;
        }
        None => std::future::pending().await,
    }
}

async fn do_reload(config_file: &str, state: &Arc<ArcSwap<AppState>>) {
    let new_config = match AppConfig::from_config_file(config_file) {
        Ok(c) => Arc::new(c),
        Err(e) => {
            warn!("Failed to reload config: {}", e);
            return;
        }
    };
    match BlogService::new(new_config.clone()) {
        Ok(new_service) => {
            state.store(Arc::new(AppState {
                app_config: new_config,
                blog_service: Arc::new(new_service),
            }));
            info!("Blog reloaded successfully");
        }
        Err(e) => warn!("Failed to reload blog service: {}", e),
    }
}

async fn watch_for_changes(config_file: String, state: Arc<ArcSwap<AppState>>, interval_secs: u64) {
    let debounce = Duration::from_secs(interval_secs);
    let config_path = PathBuf::from(&config_file);

    'outer: loop {
        {
            let s = state.load();
            if !s.app_config.watch.enabled {
                info!("Watching disabled, stopping watcher");
                return;
            }
        }

        let posts_dir = {
            let s = state.load();
            PathBuf::from(&s.app_config.posts_dir)
        };

        let (tx, mut rx) = tokio::sync::mpsc::channel::<notify::Result<notify::Event>>(100);

        let mut watcher: RecommendedWatcher = match notify::recommended_watcher(move |res| {
            // try_send: drop events if channel is full rather than blocking notify's
            // internal thread. Safe because we debounce and reload once after the burst.
            let _ = tx.try_send(res);
        }) {
            Ok(w) => w,
            Err(e) => {
                warn!("Failed to create file watcher: {}", e);
                tokio::time::sleep(debounce).await;
                continue 'outer;
            }
        };

        if let Err(e) = watcher.watch(&posts_dir, RecursiveMode::Recursive) {
            warn!("Failed to watch {:?}: {}", posts_dir, e);
            tokio::time::sleep(debounce).await;
            continue 'outer;
        }

        if let Err(e) = watcher.watch(&config_path, RecursiveMode::NonRecursive) {
            warn!("Failed to watch {:?}: {}", config_path, e);
        }

        // Watch parents of any symlink components so atomic swaps (e.g. git-sync)
        // fire an event even before the old inode is deleted.
        // Only create the fallback timer if a symlink's parent is the filesystem
        // root (unwatchable), which is needed for paths like `/foo`.
        let (symlink_parent_dirs, needs_fallback) = symlink_parents(&posts_dir);
        for parent in symlink_parent_dirs {
            if let Err(e) = watcher.watch(&parent, RecursiveMode::NonRecursive) {
                warn!("Failed to watch symlink parent {:?}: {}", parent, e);
            }
        }

        info!("Watching {:?} and {:?} for changes", posts_dir, config_path);

        let canonical_at_setup = posts_dir.canonicalize().ok();
        let mut fallback: Option<tokio::time::Interval> = if needs_fallback {
            let mut i = tokio::time::interval(Duration::from_secs(30));
            i.tick().await; // consume the immediate first tick
            Some(i)
        } else {
            None
        };

        loop {
            tokio::select! {
                event = rx.recv() => {
                    match event {
                        Some(Ok(e)) => {
                            // Skip pure Access events — inotify generates these when the
                            // recursive watcher setup traverses directories via readdir,
                            // which would cause a reload loop if not filtered.
                            if matches!(e.kind, notify::EventKind::Access(_)) {
                                continue;
                            }
                        }
                        Some(Err(e)) => {
                            warn!("Watch error: {}", e);
                            continue 'outer;
                        }
                        None => continue 'outer,
                    }
                }
                _ = fallback_tick(&mut fallback) => {
                    // Fallback for root-level symlinks: if the canonical path changed,
                    // reload and re-establish the watcher.
                    if posts_dir.canonicalize().ok() != canonical_at_setup {
                        info!("Canonical path changed, reloading blog...");
                        do_reload(&config_file, &state).await;
                        continue 'outer;
                    }
                    continue;
                }
            }

            // Check enabled after each event
            {
                let s = state.load();
                if !s.app_config.watch.enabled {
                    info!("Watching disabled, stopping watcher");
                    return;
                }
            }

            // Debounce: drain all events that arrive within the debounce window
            loop {
                match tokio::time::timeout(debounce, rx.recv()).await {
                    Ok(Some(Ok(_))) => {} // more events — keep draining
                    Ok(Some(Err(e))) => {
                        warn!("Watch error during debounce: {}", e);
                        break;
                    }
                    Ok(None) | Err(_) => break, // channel closed or quiet period reached
                }
            }

            info!("Change detected, reloading blog...");
            do_reload(&config_file, &state).await;

            // Re-establish the watcher only if the canonical path changed (git-sync
            // symlink swap). Always restarting caused an infinite reload loop:
            // creating a recursive inotify watcher traverses directories via readdir,
            // which generates Access events that were caught as "changes".
            if posts_dir.canonicalize().ok() != canonical_at_setup {
                continue 'outer;
            }
            // Canonical path unchanged — keep the existing watcher running.
        }
    }
}

pub trait AppDepot {
    fn app(&self) -> &App;
    fn app_state(&self) -> Arc<AppState>;
}

impl AppDepot for Depot {
    fn app(&self) -> &App {
        self.obtain::<App>().unwrap()
    }

    fn app_state(&self) -> Arc<AppState> {
        self.app().state.load_full()
    }
}

pub use salvo::prelude::*;

pub mod errors;

mod assets;
pub mod healthcheck;
mod home;
mod posts;
mod robots;
mod rss;

pub fn router() -> Router {
    Router::new()
        .push(assets::routes())
        .push(home::routes())
        .push(posts::routes())
        .push(rss::routes())
        .push(robots::routes())
        .push(healthcheck::routes())
}

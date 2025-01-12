pub use salvo::prelude::*;

pub mod errors;

mod assets;
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
}

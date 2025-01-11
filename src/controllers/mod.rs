pub use salvo::prelude::*;

mod assets;
mod home;
mod posts;

pub fn router() -> Router {
    Router::new()
        .push(assets::routes())
        .push(home::routes())
        .push(posts::routes())
}

pub use salvo::prelude::*;

mod assets;
mod home;

pub fn router() -> Router {
    Router::new().push(assets::routes()).push(home::routes())
}

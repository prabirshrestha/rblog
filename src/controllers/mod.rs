pub use salvo::prelude::*;

mod home;

pub fn router() -> Router {
    Router::new()
        .push(home::routes())
}

pub use salvo::prelude::*;

pub fn routes() -> Router {
    Router::new().get(home)
}

#[handler]
async fn home(req: &mut Request, res: &mut Response) {
    res.render(Text::Html("Hello World"));
}

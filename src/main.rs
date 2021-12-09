mod app;
mod appstate;
mod blog;
mod handlers;
mod markdown;
mod routes;

use app::app;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

fn main() {
    pretty_env_logger::init();
    trillium_smol::run(app());
}

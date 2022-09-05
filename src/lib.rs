pub mod appstate;
pub mod blog;
pub mod handlers;
pub mod markdown;
pub mod routes;
pub mod server;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

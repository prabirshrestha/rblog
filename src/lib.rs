pub mod app;
pub mod appstate;
pub mod blog;
pub mod handlers;
pub mod markdown;
pub mod routes;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

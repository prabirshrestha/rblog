pub mod app;
pub mod app_config;
pub mod appstate;
pub mod blog;
pub mod cli;
pub mod controllers;
pub mod markdown;
pub mod utils;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

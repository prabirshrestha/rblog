pub mod app;
pub mod app_config;
pub mod cli;
pub mod controllers;
pub mod models;
pub mod services;
pub mod utils;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

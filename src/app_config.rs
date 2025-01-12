use schematic::{Config, ConfigLoader};
use serde::{Deserialize, Serialize};

#[derive(Debug, Config, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppConfig {
    #[setting(default = "127.0.0.1", env = "HOST")]
    pub host: String,

    #[setting(default = "8080", env = "PORT")]
    pub port: String,

    #[setting]
    pub title: String,
    #[setting]
    pub root: String,
    #[setting]
    pub page_size: Option<usize>,
    #[setting]
    pub enable_drafts: Option<bool>,
    #[setting(default = "posts", env = "POSTS_DIR")]
    pub posts_dir: String,
    #[setting]
    pub github: Option<String>,
    #[setting]
    pub mastodon: Option<String>,
    #[setting]
    pub twitter: Option<String>,
    #[setting]
    pub disqus: Option<String>,
    #[setting]
    pub giscus: Option<Giscus>,
    #[setting]
    pub google_analytics: Option<GoogleAnalytics>,
    #[setting]
    pub syntax_highlight: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Config, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Giscus {
    #[setting]
    pub script_src: String,
    #[setting]
    pub repo: String,
    #[setting]
    pub repo_id: String,
    #[setting]
    pub category: String,
    #[setting]
    pub category_id: String,
    #[setting]
    pub mapping: String,
    #[setting]
    pub reactions_enabled: u32,
    #[setting]
    pub emit_metadata: u32,
    #[setting]
    pub theme: String,
    #[setting]
    pub lang: String,
    #[setting]
    pub crossorigin: String,
}

#[derive(Debug, Clone, PartialEq, Config, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleAnalytics {
    #[setting]
    pub ga_measurement_id: String,
}

impl AppConfig {
    pub fn from_config_file(path: &str) -> anyhow::Result<Self> {
        let result = ConfigLoader::<AppConfig>::new().file(path)?.load()?;
        Ok(result.config)
    }
}

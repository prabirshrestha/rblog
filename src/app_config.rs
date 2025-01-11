use schematic::{Config, ConfigLoader};

#[derive(Debug, Config)]
pub struct AppConfig {
    #[setting(default = "127.0.0.1", env = "HOST")]
    pub host: String,

    #[setting(default = "8080", env = "PORT")]
    pub port: String,
}

impl AppConfig {
    pub fn from_path() -> anyhow::Result<Self> {
        let result = ConfigLoader::<AppConfig>::new().load()?;

        Ok(result.config)
    }
}

use std::sync::Arc;

use argh::FromArgs;

use crate::{app::App, app_config::AppConfig};

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "rblog")]
pub struct Cli {
    #[argh(subcommand)]
    pub subcommand: Option<CliSubcommand>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum CliSubcommand {
    Version(SubcommandVersion),
    Run(SubcommandRun),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    subcommand,
    name = "version",
    description = "display version information"
)]
pub struct SubcommandVersion {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run", description = "run rblog")]
pub struct SubcommandRun {
    #[argh(
        option,
        short = 'c',
        description = "path to config file",
        default = "String::from(\"blog.yaml\")"
    )]
    config_file: String,
}

impl Cli {
    pub fn from_env() -> Self {
        argh::from_env()
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.subcommand {
            Some(subcommand) => match subcommand {
                CliSubcommand::Version(_) => {
                    println!("{}", App::version());
                }
                CliSubcommand::Run(args) => {
                    let config = AppConfig::from_config_file(&args.config_file)?;
                    App::from_config(Arc::new(config)).await?.run().await?;
                }
            },
            None => {
                App::from_env().await?.run().await?;
            }
        }

        Ok(())
    }
}

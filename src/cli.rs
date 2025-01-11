use argh::FromArgs;

use crate::app::App;

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
pub struct SubcommandRun {}

impl Cli {
    pub fn from_env() -> Self {
        argh::from_env()
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.subcommand {
            Some(subcommand) => match subcommand {
                CliSubcommand::Version(_) => {
                    println!("{}", App::version());
                    return Ok(());
                }
                CliSubcommand::Run(_) => {}
            },
            None => {}
        }

        App::new_from_env().await?.run().await?;

        Ok(())
    }
}

use rblog::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    Cli::from_env().run().await?;

    Ok(())
}

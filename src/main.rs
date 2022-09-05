use rblog::server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    server::run().await?;
    Ok(())
}

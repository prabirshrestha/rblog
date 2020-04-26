use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("Hello world") });
    app.listen("0.0.0.0:3000").await?;
    Ok(())
}

use anyhow::Result;
use std::env;

#[async_std::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse::<u16>()?;

    let addr = format!("0.0.0.0:{}", port);

    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("Hello world") });
    app.listen(addr).await?;

    Ok(())
}

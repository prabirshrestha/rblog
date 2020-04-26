use anyhow::Result;
use std::env;
use tide::{Response, StatusCode};

#[async_std::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse::<u16>()?;

    let addr = format!("0.0.0.0:{}", port);

    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("Hello world") });
    app.at("*").all(|_| async {
        Ok(Response::new(StatusCode::NotFound).body_string(String::from("not found")))
    });
    app.listen(addr).await?;

    Ok(())
}

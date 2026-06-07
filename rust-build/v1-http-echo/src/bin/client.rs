// Simple POST method in Axum
use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:4000")
        .body("hello world")
        .send()
        .await?;

    let text = response.text().await?;
    println!("client received: {text}");
    Ok(())
}

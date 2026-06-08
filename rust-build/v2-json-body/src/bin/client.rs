use serde::Serialize;

#[derive(Serialize)]
struct Payload {
    greeting: String,
    from: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let payload = Payload {
        greeting: "hello".to_string(),
        from: "client v2".to_string(),
    };
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:4000")
        .json(&payload)
        .send()
        .await?;

    let body = response.text().await?;
    println!("client received: {body}");
    Ok(())
}

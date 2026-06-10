// EP 2 Scene 2 — v1: HTTP client that POSTs a string and reads the echo
// (mirror of v1-http-echo/client.js). reqwest is the client (the `fetch` analog);
// axum is only ever the server side.
use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Build once; reuses a connection pool. (Here we only make one request.)
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:4000")
        .body("hello world") // a raw text body — no JSON yet (that's v2)
        .send()
        .await?; // network round-trip: connect, send, await the response headers
    let text = response.text().await?; // await + read the response body as text
    println!("client received: {text}");
    Ok(())
}

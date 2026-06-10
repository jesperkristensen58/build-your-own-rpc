// EP 2 Scene 3 — v2: client that POSTs a JSON body and reads the JSON response
// (mirror of v2-json-body/client.js).
use serde::Serialize;

// #[derive(Serialize)] lets serde turn this struct into JSON. Serialize is enough
// for *sending*; reading a typed response back would also need Deserialize.
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
        .json(&payload) // serializes payload to JSON AND sets content-type
        .send()
        .await?; // network round-trip
    // .text() reads the raw response body as a String. (You could use .json() to
    // parse it into a type instead — v3's client does exactly that.)
    let body = response.text().await?;
    println!("client received: {body}");
    Ok(())
}

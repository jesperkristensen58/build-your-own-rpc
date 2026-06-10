// EP 2 Scene 5 — v4: client that reacts to error responses
// (mirror of v4-errors/client.js). Runs three calls — add (happy path),
// nope (method not found), bad (handler error) — printing the result or the error.
use reqwest::Client;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicU64, Ordering};

// Module-level request-id counter (atomic = thread-safe `let nextId = 1`).
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Each `match` is a try/catch: Ok arm = the result, Err arm = the failure message.
    match call(&client, "add", json!([1, 2])).await {
        Ok(result) => println!("add(1,2) => {result}"),
        Err(e) => eprintln!("add failed: {e}"),
    }

    match call(&client, "nope", json!([])).await {
        Ok(result) => println!("nope() => {result}"),
        Err(e) => eprintln!("nope failed: {e}"),
    }

    match call(&client, "bad", json!([])).await {
        Ok(result) => println!("bad() => {result}"),
        Err(e) => eprintln!("bad failed: {e}"),
    }
}

// Single JSON-RPC call. Returns Box<dyn Error> so it can carry BOTH a network/parse
// failure (bubbled up by `?`) AND a server-sent error envelope (Err(message.into())).
async fn call(client: &Client, method: &str, params: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let request = json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params});
    let http_response = client.post("http://127.0.0.1:4000").json(&request).send().await?;
    let response: Value = http_response.json().await?; // `?` handles network/parse failure
    // Did the server reply with an error envelope? If so, "throw" its message.
    if let Some(error) = response.get("error") {
        let message = error["message"].as_str().unwrap_or("unknown error");
        return Err(message.into());
    }
    Ok(response["result"].clone())
}

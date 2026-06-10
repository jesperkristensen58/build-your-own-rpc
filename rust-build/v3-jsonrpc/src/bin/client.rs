// EP 2 Scene 4 — v3: JSON-RPC 2.0 client (mirror of v3-jsonrpc/client.js)
//   Demonstrates a single call AND a batch. The batch shows why `id` matters:
//   results come back in completion order, but each carries its request id, so
//   the client can match each response back to the call that produced it.
use reqwest::Client;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicU64, Ordering};

// Module-level request-id counter. Rust won't allow a plain mutable global (data
// race risk), so we use an atomic — the thread-safe version of `let nextId = 1`.
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Build ONE client and lend it (&client) to both calls so they reuse its
    // connection pool — a fresh Client::new() per call would waste that setup.
    let client = reqwest::Client::new();

    // --- single call ---
    let sum = call(&client, "add", json!([2, 3])).await?;
    println!("single add(2, 3) => {sum}");

    // --- batch: slow + fast in one POST; watch the id field do its job ---
    let results = batch_call(
        &client,
        json!([
            { "jsonrpc": "2.0", "id": 1, "method": "slow", "params": [500] },
            { "jsonrpc": "2.0", "id": 2, "method": "add",  "params": [1, 2] }
        ]),
    )
    .await?;
    println!("batch results: {results}");
    Ok(())
}

// Single JSON-RPC call: build the envelope, POST it, return just the `result`.
async fn call(client: &Client, method: &str, params: Value) -> Result<Value, reqwest::Error> {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed); // atomic `nextId++`
    let request = json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params});
    let response = client
        .post("http://127.0.0.1:4000")
        .json(&request) // serializes the body AND sets content-type: application/json
        .send()
        .await?; // network round-trip: connect, send, await the response headers
    let body: Value = response.json().await?; // await the body bytes, then parse JSON
    // Index into the untyped reply and clone the value out — you can't move out of a
    // borrow (indexing returns &Value), so clone copies it. Tiny cost, noted but fine.
    Ok(body["result"].clone())
}

// Batch call: send the pre-built array as-is and return the whole parsed reply.
// No id generation or result extraction here — the caller already put an id in
// each request, and we hand back the entire response array.
async fn batch_call(client: &Client, requests: Value) -> Result<Value, reqwest::Error> {
    // The trailing `.json().await` already produces Result<Value, reqwest::Error>
    // — exactly our return type — so no `?`/`Ok()` wrapper is needed (tail expression).
    client
        .post("http://127.0.0.1:4000")
        .json(&requests)
        .send()
        .await?
        .json()
        .await
}

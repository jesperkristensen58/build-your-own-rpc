use reqwest::Client;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let sum = call(&client, "add", json!([2, 3])).await?;
    println!("single add(2, 3) => {sum}");

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

async fn call(client: &Client, method: &str, params: Value) -> Result<Value, reqwest::Error> {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let request = json!({"jsonrpc": "2.0", "id": id,"method": method,"params": params});
    let response = client
        .post("http://127.0.0.1:4000")
        .json(&request)
        .send()
        .await?;
    let body: Value = response.json().await?;
    Ok(body["result"].clone()) //introduces compute latency
}

async fn batch_call(client: &Client, requests: Value) -> Result<Value, reqwest::Error> {
    client
        .post("http://127.0.0.1:4000")
        .json(&requests)
        .send()
        .await?
        .json()
        .await
}

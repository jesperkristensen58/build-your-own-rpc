// EP 2 Scene 5 — v4: JSON-RPC 2.0 server with error envelopes
// (mirror of v4-errors/server.js). Inherits v3 (dispatch + batch + concurrency)
// and adds JSON-RPC 2.0 error codes:
//   -32601 method not found · -32603 handler error · -32700 parse error
use axum::{Router, Json, routing::post};
use serde_json::{json, Value};
use futures_util::stream::{FuturesUnordered, StreamExt};

#[tokio::main]
async fn main() {
    // One route: POST / -> handle. `post(handle)` passes the fn by value (no parens).
    let app = Router::new().route("/", post(handle));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000").await.unwrap();
    println!("server v5 listening on http://127.0.0.1:4000");
    // The accept loop — this await runs for the lifetime of the server.
    axum::serve(listener, app).await.unwrap();
}

// Takes the body as a raw `String`, NOT axum's Json<Value> extractor. The extractor
// would auto-reject invalid JSON with an HTTP 400 before we ever run — so we could
// never return a -32700 envelope. Taking the String lets us parse it ourselves.
async fn handle(body: String) -> Json<Value> {
    println!("server received: {body}");

    // Parse manually so a parse failure becomes a JSON-RPC error, not an HTTP 400.
    // `id` is null: if the body isn't valid JSON, there's no id to echo back.
    let parsed: Value = match serde_json::from_str(&body) {
        Ok(value) => value,
        Err(_) => {
            return Json(json!({
                "jsonrpc": "2.0",
                "id": null,
                "error": { "code": -32700, "message": "Parse error" }
            }));
        }
    };

    match parsed {
        // --- batch: an array of requests in one POST (run concurrently) ---
        Value::Array(requests) => {
            // Build the futures without awaiting, then drive them all together;
            // results land in completion order, each tagged with its id.
            let mut futures: FuturesUnordered<_> =
                requests.iter().map(|req| process(req)).collect();
            let mut results = Vec::new();
            while let Some(result) = futures.next().await {
                results.push(result);
            }
            Json(json!(results))
        }
        // --- single request: process once, return the bare object ---
        single => Json(process(&single).await),
    }
}

// Turn one JSON-RPC request into one JSON-RPC response — a result OR an error envelope.
async fn process(req: &Value) -> Value {
    let id = req["id"].clone();
    let method = req["method"].as_str().unwrap_or(""); // unwrap_or: a bad method -> -32601, not a panic
    let params = &req["params"];

    // Rust has no exceptions: instead of try/catch, each arm RETURNS its outcome as a
    // Result — Ok(result) or Err((code, message)). Errors are values, not throws.
    let outcome: Result<Value, (i64, String)> = match method {
        "add" => Ok(json!(params[0].as_i64().unwrap() + params[1].as_i64().unwrap())),
        "echo" => Ok(params[0].clone()),
        "whoami" => Ok(json!("server v5")),
        "slow" => {
            // Async handler: sleep, then reply — the tokio equivalent of
            // `await new Promise(r => setTimeout(r, ms))`.
            let ms = params[0].as_u64().unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
            Ok(json!(format!("slept {ms}ms")))
        },
        "hang" => Ok(std::future::pending().await),
        "bad" => Err((-32603, "intentional failure".to_string())), // the handler "throws"
        _ => Err((-32601, format!("Method not found: {method}"))), // no such handler
    };

    // Turn the outcome into the JSON-RPC envelope: Ok -> result, Err -> error object.
    match outcome {
        Ok(result) => json!({ "jsonrpc": "2.0", "id": id, "result": result }),
        Err((code, message)) => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": { "code": code, "message": message }
        }),
    }
}

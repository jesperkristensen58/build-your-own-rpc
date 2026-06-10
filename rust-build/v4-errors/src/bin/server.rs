

// EP 2 Scene 4 — v3: JSON-RPC 2.0 server (mirror of v3-jsonrpc/server.js)
//   - JSON-RPC envelope (jsonrpc, method, params, id)
//   - Method dispatch via `match`
//   - Batch support (a single POST may carry an array of requests)
//   - Requests run concurrently; each response echoes its `id` so the client can
//     correlate results that arrive in completion order, not request order
use axum::{Router, Json, routing::post};
use serde_json::{json, Value};
use futures_util::stream::{FuturesUnordered, StreamExt};

#[tokio::main]
async fn main() {
    // One route: POST / -> handle. `post(handle)` passes the fn by value (no parens).
    let app = Router::new().route("/", post(handle));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000").await.unwrap();
    println!("server v4 listening on http://127.0.0.1:4000");
    // The accept loop — this await runs for the lifetime of the server.
    axum::serve(listener, app).await.unwrap();
}

// `Json<Value>` reads the whole request body off the socket and parses it before
// this function even starts — the largest latency here, hidden in the signature.
// `Value` holds EITHER an object (single call) or an array (batch), so one type
// covers both shapes.
async fn handle(body: String) -> Json<Value> {
    println!("server received: {body}");

    let parsed: Value = match serde_json::from_str(&body) {
        Ok(value) => value,
        Err(_) => return Json(json!({"jsonrpc": "2.0",
            "id": null,
            "error": { "code": -32700, "message": "Parse error" }}))
    };

    match parsed {
        // --- batch: an array of requests in one POST ---
        Value::Array(requests) => {
            // Build all the futures WITHOUT awaiting — calling process(req) only
            // constructs each future; nothing runs yet. FuturesUnordered then drives
            // them all concurrently.
            let mut futures: FuturesUnordered<_> =
                requests.iter().map(|req| process(req)).collect();

            // Vec::new() itself is free (no allocation); growth would cost ~log n
            // reallocs at the push() calls. We know the size, so could preallocate
            // with Vec::with_capacity(requests.len()) to avoid that.
            let mut results = Vec::new();
            // THE real latency: `.next().await` yields each result as it COMPLETES
            // (so fast calls land before slow ones) and waits on the handlers,
            // including `slow`'s sleep.
            while let Some(result) = futures.next().await {
                results.push(result);
            }
            Json(json!(results)) // array in -> array out
        }
        // --- single request: not an array; process once, return the bare object ---
        single => Json(process(&single).await),
    }
}

// Turn one JSON-RPC request into one JSON-RPC response.
// Takes &Value (a borrow) so the batch arm can call it per element without
// consuming the array; returns an owned Value.
async fn process(req: &Value) -> Value {
    let id = req["id"].clone(); // echo the id back so the client can correlate
    let method = req["method"].as_str().unwrap_or("");
    let params = &req["params"];

    // Dispatch on the method name — the Rust equivalent of JS's `handlers[method]`.
    // Every arm yields a `Value`, so the match has one consistent type.
    let outcome: Result<Value, (i64, String)> = match method {
        "add" => Ok(json!(params[0].as_i64().unwrap() + params[1].as_i64().unwrap())),
        "echo" => Ok(params[0].clone()),
        "whoami" => Ok(json!("server v3")),
        "slow" => {
            // The async handler: sleep, then reply — the equivalent of
            // `await new Promise(r => setTimeout(r, ms))` in the JS version.
            let ms = params[0].as_u64().unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
            Ok(json!(format!("slept {ms}ms")))
        },
        "bad" => Err((-32603, "intentional failure".to_string())),
        _ => Err((-32601, format!("Method not found: {method}"))), // unknown method — v4 turns this into a real JSON-RPC error
    };

    match outcome {
        Ok(result) =>  json!({ "jsonrpc": "2.0", "id": id, "result": result }),
        Err((code, message)) => json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}})

    }

    // The JSON-RPC response envelope; `id` echoed so completion-order results stay matchable.
   
}

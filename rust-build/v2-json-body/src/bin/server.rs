// EP 2 Scene 3 — v2: HTTP server that parses a JSON body and replies with JSON
// (mirror of v2-json-body/server.js). The big change from v1: we let axum's Json
// extractor read + parse the body for us instead of hand-rolling the stream loop.
use axum::{Router, Json, routing::post};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(echo_json));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    println!("server listening on http://127.0.0.1:4000");
    axum::serve(listener, app).await.unwrap();
}

// `Json<Value>` reads the whole body and parses it as JSON — the high-level
// replacement for v1's manual `on('data')` loop. `Value` accepts any JSON shape,
// so we don't have to commit to a struct.
async fn echo_json(Json(req): Json<Value>) -> Json<Value> {
    println!("server received: {req:#?}"); // {:#?} = pretty, multi-line debug view
    // Build the reply with the json! macro — reads almost like the JS object literal
    // `{ you_said: req, server: 'v2' }`.
    let response = json!({"you_said": req, "server" : "v2"});
    Json(response) // Json<_> serializes the value AND sets content-type: application/json
}

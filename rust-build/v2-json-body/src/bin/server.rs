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

async fn echo_json(Json(req): Json<Value>) -> Json<Value> {
    println!("server received: {req:#?}");
    let response = json!({"you_said": req, "server" : "v2"});
    Json(response)

}

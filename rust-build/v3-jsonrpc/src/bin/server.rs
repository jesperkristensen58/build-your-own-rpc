use axum::{Router, Json, routing::post};
use serde_json::{json, Value};
use futures_util::stream::{FuturesUnordered, StreamExt};


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(handle));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000").await.unwrap();
    println!("server v3 listening on http://127.0.0.1:4000");
    axum::serve(listener, app).await.unwrap();
}

async fn handle(Json(parsed): Json<Value>) -> Json<Value> {
    println!("server received: {parsed}");

    match parsed {
        Value::Array(requests) => {
            // let mut results = Vec::new();
            // for req in &requests {
            //     results.push(process(req).await);
            // }
            // Json(json!(results))
            let mut futures: FuturesUnordered<_> = requests.iter().map(|req| process(req)).collect();
            let mut results = Vec::new(); // unknown size introduces latency
            while let Some(result) = futures.next().await {
                results.push(result); // logn resizing
            }
            Json(json!(results))
        },
        single => Json(process(&single).await),
    }
}


async fn process(req: &Value) -> Value {
    let id = req["id"].clone();
    let method = req["method"].as_str().unwrap();
    let params = &req["params"];

    let result = match method {
        "add" => json!(params[0].as_i64().unwrap() + params[1].as_i64().unwrap()),
        "echo" => params[0].clone(),
        "whoami" => json!("server v3"),
        "slow" => {
            let ms = params[0].as_u64().unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
            json!(format!("slept {ms}ms"))
        },
        _ => json!(null),
    };
    json!({ "jsonrpc": "2.0", "id": id, "result": result }) 
}
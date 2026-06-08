use axum::{Router, extract::Request, routing::post};
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(echo));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000").await.unwrap();
    println!("server listening on http://127.0.0.1:4000");
    axum::serve(listener, app).await.unwrap();
}



async fn echo(req: Request) -> String {
    let mut body = String::new(); 
    let mut stream = req.into_body().into_data_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        body.push_str(&String::from_utf8_lossy(&chunk));
    }
    println!("server received: {body}");
    format!("echo: {body}")
}
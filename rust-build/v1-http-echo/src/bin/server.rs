// EP 2 Scene 2 — v1: HTTP server that echoes the request body
// (mirror of v1-http-echo/server.js). Done the "from scratch" way: we read the
// body stream by hand — the faithful analog of Node's `let body=''; req.on('data')`
// — rather than using an axum extractor (that comes in v2).
use axum::{Router, extract::Request, routing::post};
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    // One route: POST / -> echo. `post(echo)` passes the fn by value (no parens).
    let app = Router::new().route("/", post(echo));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000").await.unwrap();
    println!("server listening on http://127.0.0.1:4000");
    // The accept loop — this await runs for the lifetime of the server.
    axum::serve(listener, app).await.unwrap();
}

// Takes the raw `Request` so we can stream the body ourselves.
async fn echo(req: Request) -> String {
    let mut body = String::new(); // empty String — no heap allocation until we push
    let mut stream = req.into_body().into_data_stream();

    // `.next().await` is the real latency: each iteration waits for the next body
    // chunk to arrive over the network — the equivalent of the `on('data')` event.
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        // from_utf8_lossy borrows valid UTF-8 (usually no alloc); push_str copies it
        // in, growing `body` with ~log n reallocations as it accumulates.
        body.push_str(&String::from_utf8_lossy(&chunk));
    }
    // Falling out of the loop = the stream is exhausted — the `on('end')` event.
    println!("server received: {body}");
    format!("echo: {body}") // String implements IntoResponse -> a text/plain reply
}

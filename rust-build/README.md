# Rust Port — Build Your Own RPC

A faithful Rust port of the Node.js tutorial in this repo (Episode 2, scenes v1–v5).
Each stage mirrors its `vN-*/` JavaScript counterpart as closely as the language
allows, so the two can be read side by side.

The goal is pedagogical: keep the *shape* of the Node code while showing the
idiomatic Rust equivalent of each concept (async I/O, JSON, dispatch, errors,
timeouts).

## Layout

A single Cargo **workspace**, one package per tutorial stage, each exposing a
`server` and a `client` binary:

```
rust-build/
├── Cargo.toml            # workspace + shared dependency versions
├── v1-http-echo/         # raw HTTP echo
├── v2-json-body/         # JSON request/response
├── v3-jsonrpc/           # JSON-RPC 2.0: ids, dispatch, batch, concurrency
├── v4-errors/            # JSON-RPC error envelopes
└── v5-timeouts/          # client-side timeouts
```

## Running

Server and client are separate processes — run each in its own terminal:

```bash
# terminal 1 — start the server
cargo run -p v3-jsonrpc --bin v3-server

# terminal 2 — run the client
cargo run -p v3-jsonrpc --bin v3-client
```

Binaries are uniquely named per stage (`v1-server` … `v5-client`), so
`cargo run --bin v3-server` also works without `-p`.

## What each stage covers

| Stage | Mirrors | Adds |
|-------|---------|------|
| **v1** | `v1-http-echo` | Raw HTTP echo. Body read **by hand** from the stream (the faithful analog of Node's `req.on('data')`). |
| **v2** | `v2-json-body` | JSON in/out via axum's `Json` extractor and the `serde_json::json!` macro. |
| **v3** | `v3-jsonrpc` | JSON-RPC 2.0: an atomic request-id counter, `match`-based method dispatch, batch requests, and **concurrent** execution that returns results in completion order. |
| **v4** | `v4-errors` | Per-request error envelopes (`-32601` method not found, `-32603` handler error, `-32700` parse error). |
| **v5** | `v5-timeouts` | Client-side timeouts that cancel an in-flight request. |

## Design decisions

A few places where the Rust port deliberately diverges from a naive translation,
and why:

- **Cargo workspace, one package per stage.** Mirrors the per-folder structure of
  the Node repo, gives each stage its own `Cargo.toml` (so v3 can add a dependency
  without affecting v1), and shares a single `Cargo.lock` and `target/` for fast
  builds.

- **`src/bin/{server,client}.rs` with explicit, unique names.** Each package sets
  `autobins = false` and declares `[[bin]]` entries named `vN-server` / `vN-client`.
  Without this, every package's default `server`/`client` binaries collide on the
  shared `target/` path and Cargo warns. Unique names keep `cargo build` across the
  whole workspace clean.

- **axum for servers, reqwest for clients.** axum is the server framework (the
  `http.createServer` analog); reqwest is the HTTP client (the `fetch` analog).
  Both are async and built on tokio, so they compose naturally.

- **v1 reads the body by hand; v2+ uses the `Json` extractor.** v1 keeps Node's
  low-level `on('data')`/`on('end')` feel using a manual body stream, because that
  is what the first scene is teaching. From v2 on, the higher-level extractor is the
  idiomatic choice and removes the boilerplate.

- **Untyped JSON via `serde_json::Value`.** The tutorial's `JSON.parse` accepts any
  shape, so the port uses `Value` (and the `json!` macro) rather than committing to
  `#[derive(Deserialize)]` structs. This keeps the dynamic, schema-free feel of the
  original.

- **Concurrent batches with `FuturesUnordered` (v3).** The episode's point is that
  batch responses arrive in *completion* order while the `id` field keeps them
  correlatable. `FuturesUnordered` yields each result as it finishes, reproducing
  that behavior; `join_all` (which preserves input order) would not.

- **Errors as values, not exceptions (v4).** Rust has no `try/catch`, so each
  handler returns `Result<Value, (code, message)>` and a final `match` builds the
  JSON-RPC envelope. Failures are explicit return values rather than thrown
  exceptions.

- **Manual parse for `-32700` (v4).** axum's `Json` extractor auto-rejects invalid
  JSON with an HTTP 400 *before* the handler runs, which would prevent returning a
  JSON-RPC parse-error envelope. So the v4/v5 servers take the raw body as a
  `String` and call `serde_json::from_str` themselves, turning a parse failure into
  a proper `-32700` response.

- **Timeouts via `tokio::time::timeout` (v5).** The Node client uses an
  `AbortController`; the Rust client wraps the request future in
  `tokio::time::timeout`. When the timer wins, the future is dropped — and dropping
  a future cancels it, which is Rust's equivalent of aborting the request.

- **`futures-util` pinned to `=0.3.31`.** Version `0.3.32` depends on a
  `futures-macro 0.3.32` that is not published to crates.io, so resolution fails;
  the pin avoids it.

## Requirements

- A recent stable Rust toolchain (uses edition 2024).
- No running services required beyond the workspace itself; each stage's server
  listens on `127.0.0.1:4000`.

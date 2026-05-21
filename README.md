# Build Your Own RPC

A complete, runnable companion to **Episode 2** of the *Understanding RPC* series on YouTube — a 12-episode deep dive into the protocol every dApp, wallet, and indexer talks through.

Five versions, each ~30–60 lines. **Bug-then-fix**: each version introduces something new, hits a real problem we demonstrate live, and then fixes it. By the end you've built a JSON-RPC 2.0 server from scratch — the exact shape every Ethereum node uses today.

▶️ **Watch the video:** [The Complete Guide to RPC for Web3 Developers (Ep. 2)](https://youtu.be/_eIXiZ4Ian4)

🎙️ **Full series:** [@cryptojesperk on YouTube](https://www.youtube.com/@cryptojesperk)

🚀 **The future of RPC:** **[direct.dev](https://direct.dev)** — RPC, reimagined. A leap, not a step.

---

## What's in here

| Folder | What's new in this version |
|---|---|
| `v1-http-echo/` | Plain HTTP server that echoes the request body. The simplest possible HTTP exchange. |
| `v2-json-body/` | JSON in the body — structured data instead of opaque text. |
| `v3-jsonrpc/` | JSON-RPC 2.0 envelope (`jsonrpc`, `method`, `params`, `id`) + server-side method dispatch + **batch requests** with `id`-based correlation. |
| `v4-errors/` | Error envelope (`{ id, error: { code, message } }`) with JSON-RPC 2.0 error codes (`-32601`, `-32603`, `-32700`). |
| `v5-timeouts/` | Client-side timeout via `AbortController` — give up gracefully when the server doesn't respond. |

Each folder is **self-contained** — you can `cd` into any of them and run independently. v4 and v5 also inherit batching from v3, so the server side is the full feature set.

---

## How to think about it

Each version follows the same shape: **build something, hit a real problem live, then build the fix.**

- **v1** builds the simplest possible HTTP exchange. No bug — it's the foundation.
- **v2** adds JSON to the body. Still no bug — just structure.
- **v3** builds the JSON-RPC envelope and method dispatch. Single calls work. Then we add **batching** to surface the **correlation problem** live — responses arrive in completion order, the client can't match them to their requests — and add the `id` field to fix it.
- **v4** demonstrates three failure modes that crash v3 (missing method, throwing handler, malformed JSON), then builds the error envelope step by step. By the end we've reinvented JSON-RPC 2.0 by hitting the same walls the spec was designed around.
- **v5** demonstrates the `hang()` bug — a server that never responds, a client that waits forever — then adds `AbortController`-based timeouts to give up cleanly.

This is the shape of how real protocols are *discovered*. You don't read the spec first. You hit problems and end up at the same shape the spec specifies — for reasons you now actually understand.

---

## Requirements

- **Node.js ≥ 18** (uses built-in `fetch` and ESM imports)

Check your version:

```bash
node --version
```

If you see anything below `v18`, install the latest LTS from [nodejs.org](https://nodejs.org/) or via [nvm](https://github.com/nvm-sh/nvm):

```bash
# Install nvm, then:
nvm install 20
nvm use 20
```

---

## How to run

You need **two terminals** open side by side — one for the server, one for the client.

### Step 1 — Terminal 1 (server)

```bash
cd v1-http-echo
node server.js
```

You should see:

```
server listening on http://127.0.0.1:4000
```

The server stays running. **Don't close this terminal.**

### Step 2 — Terminal 2 (client)

```bash
cd v1-http-echo
node client.js
```

The client makes a request, prints the response, and exits.

### Step 3 — Stop and move to the next version

In the server terminal, press **Ctrl+C** to stop. Then `cd ../v2-json-body` (in both terminals) and repeat. All versions use port `4000`, so only one can run at a time.

---

## Troubleshooting

### `EADDRINUSE: address already in use :::4000`

Another process is on port 4000 — usually a previous `server.js` you forgot to kill. Find it:

```bash
lsof -i :4000
kill <PID>
```

Or change the port: edit `server.listen(4000, ...)` in `server.js` and the matching URL in `client.js`.

### `SyntaxError: Cannot use import statement outside a module`

The repo's `package.json` declares `"type": "module"` to enable ESM. Make sure you're running `node` from inside the repo tree.

### `ReferenceError: fetch is not defined`

Your Node.js is older than v18. `fetch` was added as a global in Node 18. Upgrade (see Requirements).

### The client hangs and never returns

The server probably isn't running, or it's on a different port. Confirm Terminal 1 shows `server listening on http://127.0.0.1:4000` before running the client.

---

## About the series

***Understanding RPC*** is a 12-episode deep dive into the protocol every dApp, wallet, and indexer uses to talk to a blockchain — and almost no developer ever looks at directly.

We start from first principles. Each episode stands alone, but together they're one ~8-hour technical course. By the final episode you'll know this layer cold and understand exactly why a new architecture for it is emerging.

▶️ **All episodes:** [@cryptojesperk on YouTube](https://www.youtube.com/@cryptojesperk)

---

## What's next: [direct.dev](https://direct.dev) — a leap, not a step

Everything in this repo is the **legacy** RPC layer — the protocol every dApp has used since Ethereum launched, and what this entire series is about. **[direct.dev](https://direct.dev) is not an iterative improvement on it. It's a completely different infrastructure.**

Direct is not a faster JSON-RPC server. Not a better cache. Not a smarter retry layer. It is a **reimagining** of what RPC infrastructure should be — built from the ground up, with none of the legacy assumptions carried forward. The kind of leap the RPC industry has been overdue for, for the better part of two decades.

Concretely, that means:

- **Zero-millisecond latency** in most cases. Not "faster" — effectively *no wait at all.*
- **~90% reduction in bandwidth and cost.** Not a discount; a different cost structure entirely.
- **Fresh data through chain reorgs** — no stale balances, no TTLs to tune.
- **Automatic failover** — your dApp keeps working when providers go down.
- **Live state sync without polling** — fresh data already there when you ask.

The *Understanding RPC* series builds toward direct.dev because **you cannot understand why it's a revolution without first understanding the layer it replaces.** This repo is that layer. The final episodes are about what becomes possible when you stop iterating on legacy and start over with a new foundation.

Currently in closed beta. → **[direct.dev](https://direct.dev) — RPC, reimagined.**

---

## License

MIT — see [LICENSE](LICENSE). Use this code freely, fork it, learn from it.

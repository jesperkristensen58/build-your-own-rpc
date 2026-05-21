// EP 2 Scene 6 — v5 (final state):
//   Identical to v4 (batching + per-request error envelope) PLUS
//   a `hang` handler that never resolves — so the client can demonstrate
//   what happens when a server-side call simply never returns.
import http from 'node:http';

const handlers = {
  add: ([a, b]) => a + b,
  echo: ([msg]) => msg,
  whoami: () => 'server v5',
  slow: async ([ms]) => {
    await new Promise(r => setTimeout(r, ms));
    return `slept ${ms}ms`;
  },
  bad: () => { throw new Error('intentional failure'); },
  hang: () => new Promise(() => {}), // intentionally never resolves
};

async function handleOneRpc({ jsonrpc, id, method, params }) {
  try {
    const handler = handlers[method];
    if (!handler) {
      return {
        jsonrpc: '2.0',
        id,
        error: { code: -32601, message: `Method not found: ${method}` },
      };
    }
    const result = await handler(params);
    return { jsonrpc: '2.0', id, result };
  } catch (err) {
    return {
      jsonrpc: '2.0',
      id,
      error: { code: -32603, message: err.message || String(err) },
    };
  }
}

const server = http.createServer((req, res) => {
  let body = '';
  req.on('data', chunk => body += chunk);
  req.on('end', async () => {
    let parsed;
    try {
      parsed = JSON.parse(body);
    } catch (err) {
      res.writeHead(200, { 'content-type': 'application/json' });
      res.end(JSON.stringify({
        jsonrpc: '2.0',
        id: null,
        error: { code: -32700, message: 'Parse error' },
      }));
      return;
    }

    const requests = Array.isArray(parsed) ? parsed : [parsed];
    const results = [];
    await Promise.all(requests.map(async (rpc) => {
      const response = await handleOneRpc(rpc);
      results.push(response);
    }));

    const responseBody = Array.isArray(parsed) ? results : results[0];
    res.writeHead(200, { 'content-type': 'application/json' });
    res.end(JSON.stringify(responseBody));
  });
});

server.listen(4000, () => console.log('server v5 on :4000'));

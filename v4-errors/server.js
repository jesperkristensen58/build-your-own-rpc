// EP 2 Scene 5 — v4 (final state):
//   Inherits batching + id from v3.
//   Adds: per-request error envelope with JSON-RPC 2.0 error codes
//     - -32601 (method not found)
//     - -32603 (internal error / handler threw)
//     - -32700 (JSON parse error at the top level)
import http from 'node:http';

const handlers = {
  add: ([a, b]) => a + b,
  echo: ([msg]) => msg,
  whoami: () => 'server v4',
  slow: async ([ms]) => {
    await new Promise(r => setTimeout(r, ms));
    return `slept ${ms}ms`;
  },
  bad: () => { throw new Error('intentional failure'); },
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
      // Parse-level error — we can't even extract an id, so id is null.
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

server.listen(4000, () => console.log('server v4 on :4000'));

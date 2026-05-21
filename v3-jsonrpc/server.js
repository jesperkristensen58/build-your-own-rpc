// EP 2 Scene 4 — v3 (final state, post-bug-fix):
//   - JSON-RPC 2.0 envelope (jsonrpc, method, params, id)
//   - Server-side method dispatch
//   - Batch support (array of requests in one POST)
//   - Responses echo the request id so the client can correlate
//     even when batch results arrive in completion order, not request order
import http from 'node:http';

const handlers = {
  add: ([a, b]) => a + b,
  echo: ([msg]) => msg,
  whoami: () => 'server v3',
  slow: async ([ms]) => {
    await new Promise(r => setTimeout(r, ms));
    return `slept ${ms}ms`;
  },
};

const server = http.createServer((req, res) => {
  let body = '';
  req.on('data', chunk => body += chunk);
  req.on('end', async () => {
    const parsed = JSON.parse(body);
    const requests = Array.isArray(parsed) ? parsed : [parsed];

    const results = [];
    await Promise.all(
      requests.map(async ({ jsonrpc, id, method, params }) => {
        const result = await handlers[method](params);
        results.push({ jsonrpc: '2.0', id, result });
      })
    );

    const responseBody = Array.isArray(parsed) ? results : results[0];
    res.writeHead(200, { 'content-type': 'application/json' });
    res.end(JSON.stringify(responseBody));
  });
});

server.listen(4000, () => console.log('server v3 on :4000'));

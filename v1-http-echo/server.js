// EP 2 Scene 2 — v1: a plain HTTP server that echoes the request body
import http from 'node:http';

const server = http.createServer((req, res) => {
  let body = '';
  req.on('data', chunk => body += chunk);
  req.on('end', () => {
    console.log('server received:', body);
    res.writeHead(200, { 'content-type': 'text/plain' });
    res.end(`echo: ${body}`);
  });
});

server.listen(4000, () => {
  console.log('server listening on http://127.0.0.1:4000');
});


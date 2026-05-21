// EP 2 Scene 3 — v2: HTTP server that parses a JSON body and replies with JSON
import http from 'node:http';

const server = http.createServer((req, res) => {
  let body = '';
  req.on('data', chunk => body += chunk);
  req.on('end', () => {
    const request = JSON.parse(body);
    console.log('server received:', request);

    const response = { you_said: request, server: 'v2' };
    res.writeHead(200, { 'content-type': 'application/json' });
    res.end(JSON.stringify(response));
  });
});

server.listen(4000, () => console.log('server v2 on :4000'));

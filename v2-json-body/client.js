// EP 2 Scene 3 — v2: a client that POSTs a JSON body and reads the JSON response
const response = await fetch('http://127.0.0.1:4000', {
  method: 'POST',
  headers: { 'content-type': 'application/json' },
  body: JSON.stringify({ greeting: 'hello', from: 'client v2' }),
});

const data = await response.json();
console.log('client received:', data);

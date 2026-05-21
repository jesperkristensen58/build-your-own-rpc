// EP 2 Scene 2 — v1: a plain HTTP client that POSTs a string and reads the echo
const response = await fetch('http://127.0.0.1:4000', {
  method: 'POST',
  body: 'hello world',
});

const text = await response.text();
console.log('client received:', text);

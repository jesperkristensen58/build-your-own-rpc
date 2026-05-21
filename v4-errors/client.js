// EP 2 Scene 5 — v4: client throws on error responses.
// Demos three error paths against the same server:
//   1. method not found (-32601)
//   2. handler throws (-32603)
//   3. happy path (returns a result)
let nextId = 1;

async function call(method, params) {
  const id = nextId++;
  const httpResponse = await fetch('http://127.0.0.1:4000', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ jsonrpc: '2.0', id, method, params }),
  });
  const response = await httpResponse.json();
  if (response.error) throw new Error(response.error.message);
  return response.result;
}

try {
  console.log('add(1, 2) =>', await call('add', [1, 2]));
} catch (err) {
  console.error('add failed:', err.message);
}

try {
  console.log('nope() =>', await call('nope', []));
} catch (err) {
  console.error('nope failed:', err.message);
}

try {
  console.log('bad() =>', await call('bad', []));
} catch (err) {
  console.error('bad failed:', err.message);
}

// EP 2 Scene 4 — v3 (final state):
//   Demonstrates both a single JSON-RPC call AND a batch.
//   The batch shows why `id` matters: responses arrive in completion order
//   (slow finishes after add), but each carries its request id, so the
//   client can match each response back to its call.
let nextId = 1;

async function call(method, params) {
  const id = nextId++;
  const response = await fetch('http://127.0.0.1:4000', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ jsonrpc: '2.0', id, method, params }),
  });
  const { result } = await response.json();
  return result;
}

async function batchCall(requests) {
  const response = await fetch('http://127.0.0.1:4000', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(requests),
  });
  return await response.json();
}

// --- single call ---
const sum = await call('add', [2, 3]);
console.log('single add(2, 3) =>', sum);

// --- batch: slow + fast, watch the id field do its job ---
const results = await batchCall([
  { jsonrpc: '2.0', id: 1, method: 'slow', params: [500] },
  { jsonrpc: '2.0', id: 2, method: 'add',  params: [1, 2] },
]);

console.log('batch results (note: order = completion order, ids identify each):');
console.log(results);

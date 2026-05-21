// EP 2 Scene 6 — v5: client adds an AbortController-based timeout to every call.
//   - Normal call returns quickly.
//   - call('hang', ...) would block forever; the timeout aborts the fetch
//     and we throw a clear `Timeout after Xms` error instead of hanging.
let nextId = 1;

async function call(method, params, { timeoutMs = 5000 } = {}) {
  const id = nextId++;
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const httpResponse = await fetch('http://127.0.0.1:4000', {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({ jsonrpc: '2.0', id, method, params }),
      signal: controller.signal,
    });
    const response = await httpResponse.json();
    if (response.error) throw new Error(response.error.message);
    return response.result;
  } catch (err) {
    if (err.name === 'AbortError') {
      throw new Error(`Timeout after ${timeoutMs}ms (id=${id})`);
    }
    throw err;
  } finally {
    clearTimeout(timer);
  }
}

// Normal call — should return quickly
try {
  console.log('add(1, 2) =>', await call('add', [1, 2]));
} catch (err) {
  console.error('add failed:', err.message);
}

// Hung call — should time out after 2 seconds
try {
  console.log('hang() with 2s timeout...');
  await call('hang', [], { timeoutMs: 2000 });
} catch (err) {
  console.error('hang failed:', err.message);
}

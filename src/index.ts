document.addEventListener('DOMContentLoaded', async () => {
  const { greet } = await import('../wasm/pkg');
  console.log('after import, before call');
  greet('JM');
  console.log('after call');
});

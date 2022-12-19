document.addEventListener('DOMContentLoaded', async () => {
  const canvas = document.querySelector('#main');
  canvas.height = canvas.clientHeight;
  canvas.width = canvas.clientWidth;
  console.log('before import');
  await import('../wasm/pkg');
  console.log('after import');
});

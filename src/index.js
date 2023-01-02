// const state = {
//   paused: false,
//   frameRef: null
// };

document.addEventListener('DOMContentLoaded', async () => {
  const canvas = document.querySelector('#main');
  window.addEventListener('resize', () => {
    canvas.height = canvas.clientHeight;
    canvas.width = canvas.clientWidth;
  });
  canvas.height = canvas.clientHeight;
  canvas.width = canvas.clientWidth;
  console.log('before import');
  await import('../wasm/pkg');
  console.log('after import');
  // let lastFrame = 0;
  // const loop = t => {
  //   if (state.paused) return;
  //   (t - lastFrame > 16.5) && tick();
  //   lastFrame = t;
  //   state.frameRef = window.requestAnimationFrame(loop);
  // };
  // loop();
});

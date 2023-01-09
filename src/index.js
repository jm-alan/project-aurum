window.state = {
  paused: false,
  squares: 100,
  squareSize: 100
};

document.addEventListener('DOMContentLoaded', async () => {
  const randomBetween = (low, high) => Math.random() * (high - low) + low;
  const canvas = document.querySelector('#main');
  window.addEventListener('resize', () => {
    canvas.height = canvas.clientHeight;
    canvas.width = canvas.clientWidth;
    window.state.fractionMult = window.state.fractionSize - 1;
    window.state.innerRectTopLeftY = canvas.clientHeight / window.state.fractionSize;
    window.state.innerRectBottomRightY = window.state.innerRectTopLeftY * window.state.fractionMult;
    window.state.innerRectTopLeftX = canvas.clientWidth / window.state.fractionSize;
    window.state.innerRectBottomRightX = window.state.innerRectTopLeftX * window.state.fractionMult;
    window.state.innerRectWidth = window.state.innerRectBottomRightX - window.state.innerRectTopLeftX;
    window.state.innerRectHeight = window.state.innerRectBottomRightY - window.state.innerRectTopLeftY;
    canvas.height = canvas.clientHeight;
    canvas.width = canvas.clientWidth;
    window.state.ceilX = window.state.innerRectBottomRightX - window.state.xOffset;
    window.state.ceilY = window.state.innerRectBottomRightY - window.state.yOffset;
  });
  window.addEventListener('wheel', e => {
    e.preventDefault();
    if (e.deltaX > 0 && window.state.squares < 1000) window.state.squares += 1;
    else if (e.deltaX < 0 && window.state.squares > 1) window.state.squares -= 1;
    if (e.deltaY > 0 && window.state.squareSize > 10) window.state.squareSize -= 5;
    else if (e.deltaY < 0 && window.state.squareSize < 5000) window.state.squareSize += 5;
  });
  const renderContext = canvas.getContext('2d');
  window.state.fractionSize = 7;
  window.state.fractionMult = window.state.fractionSize - 1;
  window.state.innerRectTopLeftY = canvas.clientHeight / window.state.fractionSize;
  window.state.innerRectBottomRightY = window.state.innerRectTopLeftY * window.state.fractionMult;
  window.state.innerRectTopLeftX = canvas.clientWidth / window.state.fractionSize;
  window.state.innerRectBottomRightX = window.state.innerRectTopLeftX * window.state.fractionMult;
  window.state.innerRectWidth = window.state.innerRectBottomRightX - window.state.innerRectTopLeftX;
  window.state.innerRectHeight = window.state.innerRectBottomRightY - window.state.innerRectTopLeftY;
  canvas.height = canvas.clientHeight;
  canvas.width = canvas.clientWidth;
  console.log('before import');
  const { draw } = await import('../wasm/pkg');
  console.log('after import');
  let lastFrame = 0;
  window.state.currentX = randomBetween(window.state.innerRectTopLeftX, window.state.innerRectBottomRightX);
  window.state.currentY = randomBetween(window.state.innerRectTopLeftY, window.state.innerRectBottomRightY);
  window.state.xOffset = randomBetween(10, 20);
  window.state.yOffset = randomBetween(10, 20);
  window.state.ceilX = window.state.innerRectBottomRightX - window.state.xOffset;
  window.state.ceilY = window.state.innerRectBottomRightY - window.state.yOffset;
  const loop = t => {
    if (window.state.paused) return window.requestAnimationFrame(loop);
    if (t - lastFrame > 16.5) {
      renderContext.fillStyle = 'white';
      renderContext.fillRect(0, 0, canvas.width, canvas.height);
      renderContext.strokeRect(
        window.state.innerRectTopLeftX,
        window.state.innerRectTopLeftY,
        window.state.innerRectWidth,
        window.state.innerRectHeight
      );
      draw(window.state.currentX, window.state.currentY, window.state.squares, window.state.squareSize);
    }
    lastFrame = t;
    if (window.state.currentX > window.state.ceilX) window.state.currentX = window.state.innerRectTopLeftX;
    else window.state.currentX += window.state.xOffset;
    if (window.state.currentY > window.state.ceilY) window.state.currentY = window.state.innerRectTopLeftY;
    else window.state.currentY += window.state.yOffset;
    window.state.frameRef = window.requestAnimationFrame(loop);
  };
  loop();
});

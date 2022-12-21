document.addEventListener('DOMContentLoaded', async () => {
  const canvas = document.querySelector('#main');
  window.addEventListener('resize', () => {
    canvas.height = canvas.clientHeight;
    canvas.width = canvas.clientWidth;
  });
  canvas.height = canvas.clientHeight;
  canvas.width = canvas.clientWidth;
  console.log('before import');
  const { draw } = await import('../wasm/pkg');
  console.log('after import');
  let segments = 128;
  let centerX = 960.0;
  let centerY = 500.0;
  let radius = 200.0;
  let angleOffset = 0;
  let lastCursorX;
  draw(segments, centerX, centerY, radius, angleOffset);
  const wheelBehaviors = {
    [true]: {
      [true] () {
        radius += 10;
        draw(segments, centerX, centerY, radius, angleOffset);
      },
      [false] () {
        radius -= 10;
        draw(segments, centerX, centerY, radius, angleOffset);
      }
    },
    [false]: {
      [true] () {
        segments < 255 && segments++;
        draw(segments, centerX, centerY, radius, angleOffset);
      },
      [false] () {
        segments > 3 && segments--;
        draw(segments, centerX, centerY, radius, angleOffset);
      }
    }
  };

  const cursorBehaviors = {
    [true] (e) {
      angleOffset += (e.clientX - lastCursorX) / 50;
      lastCursorX = e.clientX;
      draw(segments, centerX, centerY, radius, angleOffset);
    },
    [false] (e) {
      centerX = e.clientX;
      centerY = e.clientY;
      draw(segments, centerX, centerY, radius, angleOffset);
    }
  };
  document.addEventListener('wheel', e => {
    wheelBehaviors[e.shiftKey][e.deltaY < 0]();
  });
  const mouseListener = e => cursorBehaviors[e.shiftKey](e);
  document.addEventListener('mousedown', (e) => {
    if (e.shiftKey) {
      lastCursorX = e.clientX;
    } else {
      centerX = e.clientX;
      centerY = e.clientY;
    }
    draw(segments, centerX, centerY, radius, angleOffset);
    document.addEventListener('mousemove', mouseListener);
  });
  document.addEventListener('mouseup', () => {
    document.removeEventListener('mousemove', mouseListener);
  });
});

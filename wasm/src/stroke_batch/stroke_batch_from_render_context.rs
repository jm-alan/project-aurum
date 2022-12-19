use web_sys::CanvasRenderingContext2d;

use super::StrokeBatch;

impl<'canvas_lifetime> From<&'canvas_lifetime CanvasRenderingContext2d>
  for StrokeBatch<'canvas_lifetime>
{
  fn from(context: &'canvas_lifetime CanvasRenderingContext2d) -> Self {
    StrokeBatch::using(context)
  }
}

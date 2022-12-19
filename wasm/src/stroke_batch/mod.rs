mod stroke_batch_from_contiguous_shape;
mod stroke_batch_from_render_context;

use web_sys::CanvasRenderingContext2d;

use crate::{
  canvas_point::CanvasPoint,
  contiguous_shape::{
    contiguous_shape_builder::ContiguousShapeBuilder, ContiguousShape,
  },
  stroke_config::StrokeConfig,
};

#[derive(Debug)]
pub struct StrokeBatch<'context_life> {
  render_context: &'context_life CanvasRenderingContext2d,
  pub stroke_config: StrokeConfig<'context_life>,
  shapes: Vec<ContiguousShape<'context_life>>,
}

impl<'context_life> StrokeBatch<'context_life> {
  pub fn using(
    render_context: &'context_life CanvasRenderingContext2d,
  ) -> Self {
    Self {
      render_context,
      stroke_config: Default::default(),
      shapes: vec![],
    }
  }

  pub fn shape_from(
    self,
    start: &CanvasPoint,
  ) -> ContiguousShapeBuilder<'context_life> {
    (self, start).into()
  }

  pub fn draw(self) {
    for shape in self.shapes.into_iter() {
      shape.draw(self.render_context)
    }
  }
}

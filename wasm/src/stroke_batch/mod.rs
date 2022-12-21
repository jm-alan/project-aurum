mod stroke_batch_from_render_context;

use web_sys::CanvasRenderingContext2d;

use crate::{
  canvas_point::CanvasPoint,
  contiguous_shape::{builder::ContiguousShapeBuilder, ContiguousShape},
  shape_config::ShapeConfig,
};

#[derive(Debug)]
pub struct StrokeBatch<'context_life> {
  render_context: &'context_life CanvasRenderingContext2d,
  pub config: ShapeConfig,
  pub shapes: Vec<ContiguousShape>,
}

impl<'context_life> StrokeBatch<'context_life> {
  pub fn using(
    render_context: &'context_life CanvasRenderingContext2d,
  ) -> Self {
    Self {
      render_context,
      config: Default::default(),
      shapes: vec![],
    }
  }

  pub fn config(&mut self, config: ShapeConfig) -> &mut Self {
    self.config = config;
    self
  }

  pub fn shape_from(
    &mut self,
    start: CanvasPoint,
  ) -> ContiguousShapeBuilder<'context_life, '_> {
    ContiguousShapeBuilder {
      batch: self,
      start,
      segments: vec![],
      closed_shape: false,
      filled_shape: false,
      config: Default::default(),
    }
  }
}

impl StrokeBatch<'_> {
  pub fn draw(&mut self) {
    self.shapes.drain(..).for_each(|shape| shape.draw(self.render_context));
  }
}

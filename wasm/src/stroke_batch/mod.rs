mod stroke_batch_from_render_context;

use web_sys::CanvasRenderingContext2d;

use crate::{
  canvas_point::CanvasPoint,
  contiguous_shape::{
    contiguous_shape_builder::ContiguousShapeBuilder, ContiguousShape,
  },
  enums::stroke_kind::StrokeKind,
  shape_config::ShapeConfig,
  shape_segment::ShapeSegment,
};

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

  pub fn arc(
    &mut self,
    center: CanvasPoint,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
  ) -> &mut Self {
    self.shapes.push(ContiguousShape {
      segments: vec![ShapeSegment {
        coordinates: center,
        stroke_kind: StrokeKind::CircularArc(radius, start_angle, end_angle),
      }],
      ..Default::default()
    });
    self
  }

  pub fn singleton(&mut self, kind: StrokeKind) -> &mut Self {
    self.shapes.push(kind.into());
    self
  }

  pub fn custom_shape(&mut self) -> ContiguousShapeBuilder<'context_life, '_> {
    self.into()
  }
}

impl StrokeBatch<'_> {
  pub fn draw(&mut self) {
    self.shapes.drain(..).for_each(|shape| shape.draw(self.render_context));
  }
}

mod stroke_batch_from_render_context;

use web_sys::CanvasRenderingContext2d;

use crate::{
  canvas_point::CanvasPoint,
  contiguous_shape::{
    contiguous_shape_builder::ContiguousShapeBuilder, ContiguousShape,
  },
  enums::stroke_kind::StrokeKind,
  shape_config::ShapeConfig,
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
    self.singleton(StrokeKind::CircularArc(
      center,
      radius,
      start_angle,
      end_angle,
    ))
  }

  pub fn elliptical_arc(
    &mut self,
    center: CanvasPoint,
    angle: f64,
    radius_x: f64,
    radius_y: f64,
    start_angle: f64,
    end_angle: f64,
  ) -> &mut Self {
    self.singleton(StrokeKind::Ellipse(
      center,
      angle,
      radius_x,
      radius_y,
      start_angle,
      end_angle,
    ))
  }

  pub fn circle(&mut self, center: CanvasPoint, radius: f64) -> &mut Self {
    self.singleton(StrokeKind::CircularArc(
      center,
      radius,
      0.0,
      std::f64::consts::PI * 2.0,
    ))
  }

  pub fn ellipse(
    &mut self,
    center: CanvasPoint,
    angle: f64,
    radius_x: f64,
    radius_y: f64,
  ) -> &mut Self {
    self.singleton(StrokeKind::Ellipse(
      center,
      angle,
      radius_x,
      radius_y,
      0.0,
      std::f64::consts::PI * 2.0,
    ))
  }

  pub fn square(
    &mut self,
    corner_one: CanvasPoint,
    corner_two: CanvasPoint,
  ) -> &mut Self {
    self.singleton(StrokeKind::Square(corner_one, corner_two))
  }

  pub fn radial_square(
    &mut self,
    corner_one: CanvasPoint,
    angle: f64,
    radius: f64,
  ) -> &mut Self {
    self.singleton(StrokeKind::SquareVector(corner_one, angle, radius))
  }

  fn singleton(&mut self, kind: StrokeKind) -> &mut Self {
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

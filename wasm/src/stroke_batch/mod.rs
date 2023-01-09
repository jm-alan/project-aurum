mod stroke_batch_from_render_context;

use std::collections::HashMap;

use web_sys::CanvasRenderingContext2d;

use crate::{
  angular_slice::AngularSlice,
  canvas_point::CanvasPoint,
  contiguous_shape::{
    contiguous_shape_builder::ContiguousShapeBuilder, ContiguousShape,
  },
  enums::stroke_kind::StrokeKind,
  shape_config::ShapeConfig,
};

#[derive(Debug)]
pub struct StrokeBatch<'context_life> {
  render_context: &'context_life CanvasRenderingContext2d,
  pub config: ShapeConfig,
  pub shapes: HashMap<String, ContiguousShape>,
}

impl<'context_life> StrokeBatch<'context_life> {
  pub fn using(
    render_context: &'context_life CanvasRenderingContext2d,
  ) -> Self {
    Self {
      render_context,
      config: Default::default(),
      shapes: Default::default(),
    }
  }

  pub fn config(&mut self, config: ShapeConfig) -> &mut Self {
    self.config = config;
    self
  }

  pub fn arc(
    &mut self,
    name: &str,
    center: CanvasPoint,
    radius: f64,
    slice: AngularSlice,
  ) -> &mut Self {
    self.singleton(name, StrokeKind::CircularArc(center, radius, slice))
  }

  pub fn elliptical_arc(
    &mut self,
    name: &str,
    center: CanvasPoint,
    angle: f64,
    radius_x: f64,
    radius_y: f64,
    slice: AngularSlice,
  ) -> &mut Self {
    self.singleton(
      name,
      StrokeKind::Ellipse(center, angle, radius_x, radius_y, slice),
    )
  }

  pub fn circle(
    &mut self,
    name: &str,
    center: CanvasPoint,
    radius: f64,
  ) -> &mut Self {
    self.singleton(
      name,
      StrokeKind::CircularArc(center, radius, AngularSlice::circle()),
    )
  }

  pub fn ellipse(
    &mut self,
    name: &str,
    center: CanvasPoint,
    angle: f64,
    radius_x: f64,
    radius_y: f64,
  ) -> &mut Self {
    self.singleton(
      name,
      StrokeKind::Ellipse(
        center,
        angle,
        radius_x,
        radius_y,
        AngularSlice::circle(),
      ),
    )
  }

  pub fn square(
    &mut self,
    name: &str,
    corner_one: CanvasPoint,
    corner_two: CanvasPoint,
  ) -> &mut Self {
    self.singleton(name, StrokeKind::Square(corner_one, corner_two))
  }

  pub fn radial_square(
    &mut self,
    name: &str,
    corner_one: CanvasPoint,
    angle: f64,
    radius: f64,
  ) -> &mut Self {
    self.singleton(name, StrokeKind::SquareVector(corner_one, angle, radius))
  }

  fn singleton(&mut self, name: &str, kind: StrokeKind) -> &mut Self {
    self.shapes.insert(name.to_owned(), kind.into());
    self
  }

  pub fn custom_shape(&mut self) -> ContiguousShapeBuilder<'context_life, '_> {
    self.into()
  }
}

impl StrokeBatch<'_> {
  pub fn draw(&mut self) {
    self.shapes.values().for_each(|shape| shape.draw(self.render_context));
  }
}

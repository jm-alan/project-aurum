use super::ContiguousShapeBuilder;
use crate::{canvas_point::CanvasPoint, shape_config::ShapeConfig};

impl ContiguousShapeBuilder<'_, '_> {
  pub fn beginning_at(mut self, point: CanvasPoint) -> Self {
    self.start = point;
    self
  }

  pub fn config(mut self, config: ShapeConfig) -> Self {
    self.config = config;
    self
  }

  pub fn line_through(mut self, point: CanvasPoint) -> Self {
    self.segments.push(point.into());
    self
  }

  pub fn angle_through(mut self, angle_in_radians: f64, distance: f64) -> Self {
    let last_point = if !self.segments.is_empty() {
      self.segments[self.segments.len() - 1].coordinates
    } else {
      self.start
    };
    let (unit_y, unit_x) = angle_in_radians.sin_cos();
    self.segments.push(
      CanvasPoint {
        x: (unit_x * distance) + last_point.x,
        y: (unit_y * distance) + last_point.y,
      }
      .into(),
    );
    self
  }

  pub fn offset_through(mut self, offset_x: f64, offset_y: f64) -> Self {
    let last_point = if !self.segments.is_empty() {
      self.segments[self.segments.len() - 1].coordinates
    } else {
      self.start
    };
    self.segments.push(
      CanvasPoint {
        x: last_point.x + offset_x,
        y: last_point.y + offset_y,
      }
      .into(),
    );
    self
  }
}

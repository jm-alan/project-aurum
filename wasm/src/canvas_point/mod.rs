mod canvas_point_from_float_tuple;

#[derive(Debug, Default, Clone, Copy)]
pub struct CanvasPoint {
  pub x: f64,
  pub y: f64,
}

impl CanvasPoint {
  pub fn distance_to(&self, other: CanvasPoint) -> f64 {
    f64::hypot(self.x - other.x, self.y - other.y)
  }
}

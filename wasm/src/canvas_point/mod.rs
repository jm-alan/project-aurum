mod canvas_point_from_float_tuple;

#[derive(Debug, Default, Clone, Copy)]
pub struct CanvasPoint {
  pub x: f64,
  pub y: f64,
}

impl CanvasPoint {
  pub fn distance_to(&self, other: CanvasPoint) -> f64 {
    ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
  }
}

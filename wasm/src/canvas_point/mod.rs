mod canvas_point_from_float_tuple;

#[derive(Debug, Default, Clone, Copy)]
pub struct CanvasPoint {
  pub x: f64,
  pub y: f64,
}

impl CanvasPoint {
  pub fn distance_to(&self, other: CanvasPoint) -> f64 {
    f64::hypot(other.x - self.x, other.y - self.y)
  }

  pub fn angle_to(&self, other: CanvasPoint) -> f64 {
    let x_diff = other.x - self.x;
    let y_diff = other.y - self.y;
    match &(
      x_diff < 0.0,
      x_diff == 0.0,
      x_diff > 0.0,
      y_diff < 0.0,
      y_diff == 0.0,
      y_diff > 0.0,
    ) {
      (_, true, _, _, true, _) => 0.0,
      (true, _, _, _, true, _) => std::f64::consts::PI,
      (_, true, _, true, _, _) => std::f64::consts::PI * 1.5,
      (_, true, _, _, _, true) => std::f64::consts::PI * 0.5,
      (true, _, _, _, _, _) => -1.0 * (x_diff / y_diff).atan(),
      _ => (x_diff / y_diff).atan(),
    }
  }
}

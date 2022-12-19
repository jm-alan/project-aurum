use super::CanvasPoint;

impl From<(f64, f64)> for CanvasPoint {
  fn from(point: (f64, f64)) -> Self {
    Self {
      x: point.0,
      y: point.1,
    }
  }
}

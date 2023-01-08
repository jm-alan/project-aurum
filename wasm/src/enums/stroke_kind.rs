use crate::{angular_slice::AngularSlice, canvas_point::CanvasPoint};

#[derive(Debug, Default, Clone, Copy)]
pub enum StrokeKind {
  #[default]
  Line,
  Square(CanvasPoint, CanvasPoint),
  SquareVector(CanvasPoint, f64, f64),
  CircularArc(CanvasPoint, f64, AngularSlice),
  ControlledArc(CanvasPoint, f64),
  BezierCurve(CanvasPoint, CanvasPoint),
  Ellipse(CanvasPoint, f64, f64, f64, AngularSlice),
}

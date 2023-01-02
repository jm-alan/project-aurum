use crate::canvas_point::CanvasPoint;

#[derive(Debug, Default, Clone, Copy)]
pub enum StrokeKind {
  #[default]
  Line,
  Square(CanvasPoint, CanvasPoint),
  SquareVector(CanvasPoint, f64, f64),
  CircularArc(CanvasPoint, f64, f64, f64),
  ControlledArc(CanvasPoint, f64),
  BezierCurve(CanvasPoint, CanvasPoint),
  Ellipse(CanvasPoint, f64, f64, f64, f64, f64),
}

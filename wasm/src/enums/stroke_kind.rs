use crate::canvas_point::CanvasPoint;

#[derive(Debug, Default, Clone, Copy)]
pub enum StrokeKind {
  #[default]
  Line,
  CircularArc(f64, f64, f64),
  ControlledArc(CanvasPoint, f64),
  BezierCurve(CanvasPoint, CanvasPoint),
  Ellipse(f64, f64, f64, f64, f64),
}

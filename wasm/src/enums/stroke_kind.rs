use crate::canvas_point::CanvasPoint;

#[derive(Debug, Default, Clone, Copy)]
pub enum StrokeKind {
  #[default]
  Line,
  CircularArc(CanvasPoint, f64, f64, f64, bool),
  ControlledArc(CanvasPoint, f64),
  BezierCurve(CanvasPoint, CanvasPoint),
}

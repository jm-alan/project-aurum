use crate::canvas_point::CanvasPoint;

#[derive(Debug, Default, Clone, Copy)]
pub enum StrokeKind {
  #[default]
  Line,
  ControlledArc(CanvasPoint, f64),
  BezierCurve(CanvasPoint, CanvasPoint),
}

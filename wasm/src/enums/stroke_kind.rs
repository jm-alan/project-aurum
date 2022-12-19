use crate::canvas_point::CanvasPoint;

#[derive(Debug, Clone, Default)]
pub enum StrokeKind {
  #[default]
  Line,
  Arc(f64, CanvasPoint),
}

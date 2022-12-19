mod shape_segment_from_canvas_point;

use crate::{canvas_point::CanvasPoint, enums::stroke_kind::StrokeKind};

#[derive(Debug, Default, Clone, Copy)]
pub struct ShapeSegment {
  pub coordinates: CanvasPoint,
  pub stroke_kind: StrokeKind,
}

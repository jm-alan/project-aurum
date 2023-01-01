use super::ContiguousShape;
use crate::{
  enums::stroke_kind::StrokeKind, js_panic, shape_segment::ShapeSegment,
};

impl From<StrokeKind> for ContiguousShape {
  fn from(kind: StrokeKind) -> Self {
    match kind {
      StrokeKind::Line
      | StrokeKind::ControlledArc(_, _)
      | StrokeKind::BezierCurve(_, _) => {
        js_panic!(
          "Singleton shapes must be one of Square, CircularArc, or Ellipse"
        );
      }
      stroke_kind => ContiguousShape {
        segments: vec![ShapeSegment {
          stroke_kind,
          ..Default::default()
        }],
        ..Default::default()
      },
    }
  }
}

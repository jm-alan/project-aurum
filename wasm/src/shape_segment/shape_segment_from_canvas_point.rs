use super::{ShapeSegment, StrokeKind};
use crate::canvas_point::CanvasPoint;

impl From<CanvasPoint> for ShapeSegment {
  fn from(coordinates: CanvasPoint) -> Self {
    Self {
      coordinates,
      ..Default::default()
    }
  }
}

impl From<&CanvasPoint> for ShapeSegment {
  fn from(coordinates: &CanvasPoint) -> Self {
    Self {
      coordinates: coordinates.to_owned(),
      ..Default::default()
    }
  }
}

impl From<(CanvasPoint, StrokeKind)> for ShapeSegment {
  fn from(tuple: (CanvasPoint, StrokeKind)) -> Self {
    Self {
      coordinates: tuple.0,
      stroke_kind: tuple.1,
    }
  }
}

impl From<&(CanvasPoint, StrokeKind)> for ShapeSegment {
  fn from(tuple: &(CanvasPoint, StrokeKind)) -> Self {
    Self {
      coordinates: tuple.0.to_owned(),
      stroke_kind: tuple.1.to_owned(),
    }
  }
}

impl From<(&CanvasPoint, &StrokeKind)> for ShapeSegment {
  fn from(tuple: (&CanvasPoint, &StrokeKind)) -> Self {
    Self {
      coordinates: tuple.0.to_owned(),
      stroke_kind: tuple.1.to_owned(),
    }
  }
}

impl From<&(&CanvasPoint, &StrokeKind)> for ShapeSegment {
  fn from(tuple: &(&CanvasPoint, &StrokeKind)) -> Self {
    Self {
      coordinates: tuple.0.to_owned(),
      stroke_kind: tuple.1.to_owned(),
    }
  }
}

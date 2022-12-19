use super::contiguous_shape_builder::ContiguousShapeBuilder;
use crate::{canvas_point::CanvasPoint, stroke_batch::StrokeBatch};

impl<'batch_life> From<(StrokeBatch<'batch_life>, &CanvasPoint)>
  for ContiguousShapeBuilder<'batch_life>
{
  fn from(tuple: (StrokeBatch<'batch_life>, &CanvasPoint)) -> Self {
    Self {
      batch: tuple.0,
      start: tuple.1.to_owned(),
      closed_shape: false,
      filled_shape: false,
      segments: vec![],
      config: Default::default(),
    }
  }
}

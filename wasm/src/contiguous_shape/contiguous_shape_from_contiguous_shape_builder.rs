use crate::{contiguous_shape::ContiguousShape, stroke_batch::StrokeBatch};

use super::builder::ContiguousShapeBuilder;

impl<'batch_life: 'builder_life, 'builder_life>
  From<ContiguousShapeBuilder<'batch_life, 'builder_life>>
  for &'builder_life mut StrokeBatch<'batch_life>
{
  fn from(builder: ContiguousShapeBuilder<'batch_life, 'builder_life>) -> Self {
    let ContiguousShapeBuilder {
      batch,
      start,
      closed_shape,
      filled_shape,
      segments,
      intersect_type,
      config,
    } = builder;
    let shape = ContiguousShape {
      start,
      closed_shape,
      filled_shape,
      segments,
      intersect_type,
      config,
    };
    batch.shapes.push(shape);
    batch
  }
}
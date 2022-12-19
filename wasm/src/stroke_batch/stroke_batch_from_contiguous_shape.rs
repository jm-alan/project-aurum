use super::StrokeBatch;
use crate::contiguous_shape::{
  contiguous_shape_builder::ContiguousShapeBuilder, ContiguousShape,
};

impl<'context_life> From<ContiguousShapeBuilder<'context_life>>
  for StrokeBatch<'context_life>
{
  fn from(mut builder: ContiguousShapeBuilder<'context_life>) -> Self {
    let shape: ContiguousShape = (&mut builder).into();
    let mut batch = builder.batch;
    batch.shapes.push(shape);
    batch
  }
}

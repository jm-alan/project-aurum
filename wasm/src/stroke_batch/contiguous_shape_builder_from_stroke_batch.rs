use crate::contiguous_shape::builder::ContiguousShapeBuilder;

use super::StrokeBatch;

impl<'batch_life: 'builder_life, 'builder_life>
  From<&'builder_life mut StrokeBatch<'batch_life>>
  for ContiguousShapeBuilder<'batch_life, 'builder_life>
{
  fn from(batch: &'builder_life mut StrokeBatch<'batch_life>) -> Self {
    Self {
      batch,
      start: Default::default(),
      segments: Default::default(),
      closed_shape: Default::default(),
      filled_shape: Default::default(),
      config: Default::default(),
      intersect_type: Default::default(),
    }
  }
}

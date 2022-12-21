use super::ContiguousShapeBuilder;
use crate::stroke_batch::StrokeBatch;

impl<'batch_life: 'builder_life, 'builder_life>
  ContiguousShapeBuilder<'batch_life, 'builder_life>
{
  pub fn outlined(mut self) -> &'builder_life mut StrokeBatch<'batch_life> {
    self.filled_shape = false;
    self.closed_shape = true;
    self.into()
  }

  pub fn filled(mut self) -> &'builder_life mut StrokeBatch<'batch_life> {
    self.filled_shape = true;
    self.into()
  }

  pub fn fin(self) -> &'builder_life mut StrokeBatch<'batch_life> {
    self.into()
  }
}

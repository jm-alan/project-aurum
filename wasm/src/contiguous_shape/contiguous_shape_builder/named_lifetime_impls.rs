use super::{
  arc_builder::ArcBuilderStageOne, bezier_curve_builder::BZBuilderStageOne,
  ContiguousShapeBuilder,
};
use crate::stroke_batch::StrokeBatch;

impl<'batch_life: 'builder_life, 'builder_life>
  ContiguousShapeBuilder<'batch_life, 'builder_life>
{
  pub fn arc(self) -> ArcBuilderStageOne<'batch_life, 'builder_life> {
    ArcBuilderStageOne {
      parent_builder: self.into(),
      control_point: Default::default(),
    }
  }

  pub fn bezier_curve(self) -> BZBuilderStageOne<'batch_life, 'builder_life> {
    BZBuilderStageOne {
      parent_builder: self.into(),
      control_point: Default::default(),
    }
  }

  pub fn outlined(mut self) -> &'builder_life mut StrokeBatch<'batch_life> {
    self.closed_shape = true;
    self.into()
  }

  pub fn filled(mut self) -> &'builder_life mut StrokeBatch<'batch_life> {
    self.closed_shape = true;
    self.filled_shape = true;
    self.into()
  }

  pub fn fin(self) -> &'builder_life mut StrokeBatch<'batch_life> {
    self.into()
  }
}

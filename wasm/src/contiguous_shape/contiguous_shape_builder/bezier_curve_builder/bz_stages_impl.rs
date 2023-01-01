use crate::{
  canvas_point::CanvasPoint,
  contiguous_shape::contiguous_shape_builder::ContiguousShapeBuilder,
};

use super::{BZBuilderStageOne, BZBuilderStageTwo, BezierCurveBuilder};

impl<'batch_life: 'builder_life, 'builder_life>
  BZBuilderStageOne<'batch_life, 'builder_life>
{
  pub fn through_control(
    mut self,
    point: CanvasPoint,
  ) -> BZBuilderStageTwo<'batch_life, 'builder_life> {
    self.control_point = point;
    self.into()
  }
}

impl<'batch_life: 'builder_life, 'builder_life>
  BZBuilderStageTwo<'batch_life, 'builder_life>
{
  pub fn through_control(
    mut self,
    point: CanvasPoint,
  ) -> BezierCurveBuilder<'batch_life, 'builder_life> {
    self.control_point = point;
    self.into()
  }
}

impl<'batch_life: 'builder_life, 'builder_life>
  BezierCurveBuilder<'batch_life, 'builder_life>
{
  pub fn through_final(
    mut self,
    point: CanvasPoint,
  ) -> ContiguousShapeBuilder<'batch_life, 'builder_life> {
    self.final_point = point;
    self.into()
  }
}

impl<'batch_life: 'builder_life, 'builder_life>
  From<BZBuilderStageOne<'batch_life, 'builder_life>>
  for BZBuilderStageTwo<'batch_life, 'builder_life>
{
  fn from(stage_one: BZBuilderStageOne<'batch_life, 'builder_life>) -> Self {
    let BZBuilderStageOne {
      mut parent_builder,
      control_point,
    } = stage_one;
    parent_builder.control_point_one = control_point;
    Self {
      parent_builder,
      control_point: Default::default(),
    }
  }
}

impl<'batch_life: 'builder_life, 'builder_life>
  From<BZBuilderStageTwo<'batch_life, 'builder_life>>
  for BezierCurveBuilder<'batch_life, 'builder_life>
{
  fn from(stage_three: BZBuilderStageTwo<'batch_life, 'builder_life>) -> Self {
    let BZBuilderStageTwo {
      mut parent_builder,
      control_point,
    } = stage_three;
    parent_builder.final_point = control_point;
    parent_builder
  }
}

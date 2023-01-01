use crate::{
  canvas_point::CanvasPoint,
  contiguous_shape::contiguous_shape_builder::ContiguousShapeBuilder,
};

use super::{ArcBuilder, ArcBuilderStageOne, ArcBuilderStageTwo};

impl<'batch_life: 'builder_life, 'builder_life>
  ArcBuilderStageOne<'batch_life, 'builder_life>
{
  pub fn through_control(
    mut self,
    point: CanvasPoint,
  ) -> ArcBuilderStageTwo<'batch_life, 'builder_life> {
    self.control_point = point;
    self.into()
  }
}

impl<'batch_life: 'builder_life, 'builder_life>
  ArcBuilderStageTwo<'batch_life, 'builder_life>
{
  pub fn through_final(
    mut self,
    point: CanvasPoint,
  ) -> ArcBuilder<'batch_life, 'builder_life> {
    self.final_point = point;
    self.into()
  }
}

impl<'batch_life: 'builder_life, 'builder_life>
  ArcBuilder<'batch_life, 'builder_life>
{
  pub fn with_radius(
    mut self,
    radius: f64,
  ) -> ContiguousShapeBuilder<'batch_life, 'builder_life> {
    self.radius = radius;
    self.into()
  }
}

impl<'batch_life: 'builder_life, 'builder_life>
  From<ArcBuilderStageOne<'batch_life, 'builder_life>>
  for ArcBuilderStageTwo<'batch_life, 'builder_life>
{
  fn from(stage_one: ArcBuilderStageOne<'batch_life, 'builder_life>) -> Self {
    let ArcBuilderStageOne {
      mut parent_builder,
      control_point,
    } = stage_one;
    parent_builder.control_point = control_point;
    Self {
      parent_builder,
      final_point: Default::default(),
    }
  }
}

impl<'batch_life: 'builder_life, 'builder_life>
  From<ArcBuilderStageTwo<'batch_life, 'builder_life>>
  for ArcBuilder<'batch_life, 'builder_life>
{
  fn from(stage_two: ArcBuilderStageTwo<'batch_life, 'builder_life>) -> Self {
    let ArcBuilderStageTwo {
      mut parent_builder,
      final_point,
    } = stage_two;
    parent_builder.final_point = final_point;
    parent_builder
  }
}

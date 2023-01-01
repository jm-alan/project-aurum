use crate::contiguous_shape::contiguous_shape_builder::ContiguousShapeBuilder;

use super::BezierCurveBuilder;

impl<'batch_life: 'builder_life, 'builder_life>
  From<ContiguousShapeBuilder<'batch_life, 'builder_life>>
  for BezierCurveBuilder<'batch_life, 'builder_life>
{
  fn from(
    parent_builder: ContiguousShapeBuilder<'batch_life, 'builder_life>,
  ) -> Self {
    Self {
      parent_builder,
      control_one: Default::default(),
      control_two: Default::default(),
      final_point: Default::default(),
    }
  }
}

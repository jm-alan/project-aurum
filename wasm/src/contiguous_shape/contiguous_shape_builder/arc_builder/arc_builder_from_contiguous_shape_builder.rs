use crate::contiguous_shape::contiguous_shape_builder::ContiguousShapeBuilder;

use super::ArcBuilder;

impl<'batch_life: 'builder_life, 'builder_life>
  From<ContiguousShapeBuilder<'batch_life, 'builder_life>>
  for ArcBuilder<'batch_life, 'builder_life>
{
  fn from(
    parent_builder: ContiguousShapeBuilder<'batch_life, 'builder_life>,
  ) -> Self {
    Self {
      parent_builder,
      control_point: Default::default(),
      final_point: Default::default(),
      radius: Default::default(),
    }
  }
}

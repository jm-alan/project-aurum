use crate::{enums::stroke_kind::StrokeKind, shape_segment::ShapeSegment};

use super::{arc_builder::ArcBuilder, ContiguousShapeBuilder};

impl<'batch_life: 'builder_life, 'builder_life>
  From<ArcBuilder<'batch_life, 'builder_life>>
  for ContiguousShapeBuilder<'batch_life, 'builder_life>
{
  fn from(builder: ArcBuilder<'batch_life, 'builder_life>) -> Self {
    let ArcBuilder {
      mut parent_builder,
      control_point,
      final_point,
      radius,
    } = builder;
    parent_builder.segments.push(ShapeSegment {
      coordinates: final_point,
      stroke_kind: StrokeKind::Arc(control_point, radius),
    });
    parent_builder
  }
}

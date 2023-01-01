use crate::{enums::stroke_kind::StrokeKind, shape_segment::ShapeSegment};

use super::{bezier_curve_builder::BezierCurveBuilder, ContiguousShapeBuilder};

impl<'batch_life: 'builder_life, 'builder_life>
  From<BezierCurveBuilder<'batch_life, 'builder_life>>
  for ContiguousShapeBuilder<'batch_life, 'builder_life>
{
  fn from(builder: BezierCurveBuilder<'batch_life, 'builder_life>) -> Self {
    let BezierCurveBuilder {
      mut parent_builder,
      control_one,
      control_two,
      final_point,
    } = builder;
    parent_builder.segments.push(ShapeSegment {
      coordinates: final_point,
      stroke_kind: StrokeKind::BezierCurve(control_one, control_two),
    });
    parent_builder
  }
}

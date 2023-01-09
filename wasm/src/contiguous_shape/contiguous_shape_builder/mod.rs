mod anonymous_lifetime_impls;
mod arc_builder;
mod bezier_curve_builder;
mod contiguous_shape_builder_from_arc_builder;
mod contiguous_shape_builder_from_bezier_curve_builder;
mod contiguous_shape_builder_from_stroke_batch;
mod named_lifetime_impls;

use crate::{
  canvas_point::CanvasPoint, enums::intersect_type::IntersectType,
  shape_config::ShapeConfig, shape_segment::ShapeSegment,
  stroke_batch::StrokeBatch,
};

#[derive(Debug)]
pub struct ContiguousShapeBuilder<'batch_life: 'builder_life, 'builder_life> {
  pub batch: &'builder_life mut StrokeBatch<'batch_life>,
  pub name: String,
  pub start: CanvasPoint,
  pub segments: Vec<ShapeSegment>,
  pub closed_shape: bool,
  pub filled_shape: bool,
  pub intersect_type: IntersectType,
  pub config: ShapeConfig,
}

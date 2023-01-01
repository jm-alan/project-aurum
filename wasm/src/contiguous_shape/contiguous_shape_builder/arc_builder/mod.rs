mod arc_builder_from_contiguous_shape_builder;
mod arc_builder_stages_impl;

use super::ContiguousShapeBuilder;
use crate::canvas_point::CanvasPoint;

pub struct ArcBuilder<'batch_life: 'builder_life, 'builder_life> {
  pub parent_builder: ContiguousShapeBuilder<'batch_life, 'builder_life>,
  pub control_point: CanvasPoint,
  pub final_point: CanvasPoint,
  pub radius: f64,
}

pub struct ArcBuilderStageOne<'batch_life: 'builder_life, 'builder_life> {
  pub parent_builder: ArcBuilder<'batch_life, 'builder_life>,
  pub control_point: CanvasPoint,
}

pub struct ArcBuilderStageTwo<'batch_life: 'builder_life, 'builder_life> {
  parent_builder: ArcBuilder<'batch_life, 'builder_life>,
  final_point: CanvasPoint,
}

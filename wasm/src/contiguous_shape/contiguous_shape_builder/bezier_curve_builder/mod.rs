mod bz_stages_impl;

use super::ContiguousShapeBuilder;
use crate::canvas_point::CanvasPoint;

pub struct BezierCurveBuilder<'batch_life: 'builder_life, 'builder_life> {
  pub parent_builder: ContiguousShapeBuilder<'batch_life, 'builder_life>,
  pub control_point_one: CanvasPoint,
  pub control_point_two: CanvasPoint,
  pub final_point: CanvasPoint,
}

pub struct BZBuilderStageOne<'batch_life: 'builder_life, 'builder_life> {
  parent_builder: BezierCurveBuilder<'batch_life, 'builder_life>,
  control_point: CanvasPoint,
}

pub struct BZBuilderStageTwo<'batch_life: 'builder_life, 'builder_life> {
  parent_builder: BezierCurveBuilder<'batch_life, 'builder_life>,
  control_point: CanvasPoint,
}

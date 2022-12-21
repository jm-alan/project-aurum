mod anonymous_lifetime_impls;
mod named_lifetime_impls;

use crate::{
  canvas_point::CanvasPoint, shape_config::ShapeConfig,
  shape_segment::ShapeSegment, stroke_batch::StrokeBatch,
};

pub struct ContiguousShapeBuilder<'batch_life: 'builder_life, 'builder_life> {
  pub batch: &'builder_life mut StrokeBatch<'batch_life>,
  pub start: CanvasPoint,
  pub segments: Vec<ShapeSegment>,
  pub closed_shape: bool,
  pub filled_shape: bool,
  pub config: BuilderConfig,
}

#[derive(Debug, Default, Clone)]
pub enum BuilderConfig {
  #[default]
  Inherit,
  Override(ShapeConfig),
}

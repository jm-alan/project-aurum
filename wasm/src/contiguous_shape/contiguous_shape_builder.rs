use crate::{
  canvas_point::CanvasPoint, enums::stroke_kind::StrokeKind,
  shape_segment::ShapeSegment, stroke_batch::StrokeBatch,
  stroke_config::StrokeConfig,
};

pub struct ContiguousShapeBuilder<'builder_lifetime> {
  pub batch: StrokeBatch<'builder_lifetime>,
  pub start: CanvasPoint,
  pub segments: Vec<ShapeSegment>,
  pub closed_shape: bool,
  pub filled_shape: bool,
  pub config: BuilderConfig,
}

impl<'builder_lifetime> ContiguousShapeBuilder<'builder_lifetime> {
  pub fn config(mut self, config: StrokeConfig) -> Self {
    self.config = BuilderConfig::Override(config);
    self
  }

  pub fn line_through(mut self, point: &CanvasPoint) -> Self {
    self.segments.push(point.into());
    self
  }

  pub fn arc_through(
    &mut self,
    point: &CanvasPoint,
    control: &CanvasPoint,
    radius: f64,
  ) -> &mut Self {
    self
      .segments
      .push((point, &StrokeKind::Arc(radius, control.to_owned())).into());
    self
  }

  pub fn outlined(mut self) -> StrokeBatch<'builder_lifetime> {
    self.filled_shape = false;
    self.closed_shape = true;
    self.into()
  }

  pub fn filled(mut self) -> StrokeBatch<'builder_lifetime> {
    self.filled_shape = true;
    self.into()
  }

  pub fn fin(self) -> StrokeBatch<'builder_lifetime> {
    self.into()
  }
}

#[derive(Debug, Default)]
pub enum BuilderConfig {
  #[default]
  Inherit,
  Override(StrokeConfig),
}

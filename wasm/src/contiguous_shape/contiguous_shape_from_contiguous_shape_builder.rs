use super::{
  builder::{BuilderConfig, ContiguousShapeBuilder},
  ContiguousShape,
};
use crate::stroke_batch::StrokeBatch;

impl<'batch_life: 'builder_life, 'builder_life>
  From<ContiguousShapeBuilder<'batch_life, 'builder_life>>
  for &'builder_life mut StrokeBatch<'batch_life>
{
  fn from(builder: ContiguousShapeBuilder<'batch_life, 'builder_life>) -> Self {
    let ContiguousShapeBuilder {
      batch,
      start,
      closed_shape,
      filled_shape,
      segments,
      config,
    } = builder;
    let shape = ContiguousShape {
      start,
      closed_shape,
      filled_shape,
      segments,
      config: match config {
        BuilderConfig::Inherit => batch.config.clone(),
        BuilderConfig::Override(config) => config,
      },
    };
    batch.shapes.push(shape);
    batch
  }
}

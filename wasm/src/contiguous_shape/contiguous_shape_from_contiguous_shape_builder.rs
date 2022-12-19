use std::mem;

use crate::logging::console_log;

use super::{
  contiguous_shape_builder::{BuilderConfig, ContiguousShapeBuilder},
  ContiguousShape,
};

impl<'builder_life> From<&mut ContiguousShapeBuilder<'builder_life>>
  for ContiguousShape<'builder_life>
{
  fn from(builder: &mut ContiguousShapeBuilder<'builder_life>) -> Self {
    Self {
      start: mem::take(&mut builder.start),
      closed_shape: builder.closed_shape,
      filled_shape: builder.filled_shape,
      segments: mem::take(&mut builder.segments),
      config: match mem::take(&mut builder.config) {
        BuilderConfig::Inherit => {
          console_log!("Inheriting builder config");
          builder.batch.stroke_config.to_owned()
        }
        BuilderConfig::Override(config) => {
          console_log!("Overriding builder config");
          config
        }
      },
    }
  }
}

use crate::enums::{
  stroke_cap::StrokeCap, stroke_style::StrokeStyle, stroke_width::StrokeWidth,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct StrokeConfig<'config_lifetime> {
  pub style: StrokeStyle<'config_lifetime>,
  pub fill: StrokeStyle<'config_lifetime>,
  pub width: StrokeWidth,
  pub cap: StrokeCap,
}

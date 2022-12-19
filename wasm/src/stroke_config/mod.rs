use crate::enums::{
  stroke_cap::StrokeCap, stroke_style::StrokeStyle, stroke_width::StrokeWidth,
};

#[derive(Debug, Clone, Default)]
pub struct StrokeConfig {
  pub style: StrokeStyle,
  pub width: StrokeWidth,
  pub cap: StrokeCap,
}

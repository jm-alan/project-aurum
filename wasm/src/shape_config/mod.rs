use crate::enums::{
  stroke_cap::StrokeCap, stroke_style::StrokeStyle, stroke_width::StrokeWidth,
};

#[derive(Debug, Default, Clone)]
pub struct ShapeConfig {
  pub style: StrokeStyle,
  pub fill: StrokeStyle,
  pub width: StrokeWidth,
  pub cap: StrokeCap,
}

impl ShapeConfig {
  pub fn new(
    style: StrokeStyle,
    fill: StrokeStyle,
    width: StrokeWidth,
    cap: StrokeCap,
  ) -> Self {
    Self {
      style,
      fill,
      width,
      cap,
    }
  }
}

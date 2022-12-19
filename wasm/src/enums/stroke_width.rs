#[derive(Debug, Clone, Default)]
pub enum StrokeWidth {
  #[default]
  Thin,
  Medium,
  Thicc,
  Custom(f64),
}

impl From<StrokeWidth> for f64 {
  fn from(width: StrokeWidth) -> Self {
    match width {
      StrokeWidth::Thin => 1.0,
      StrokeWidth::Medium => 5.0,
      StrokeWidth::Thicc => 10.0,
      StrokeWidth::Custom(width) => width,
    }
  }
}

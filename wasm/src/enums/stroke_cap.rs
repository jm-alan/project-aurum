#[derive(Debug, Clone, Default)]
pub enum StrokeCap {
  #[default]
  Butt,
  Round,
  Square,
}

impl ToString for StrokeCap {
  fn to_string(&self) -> String {
    match &self {
      StrokeCap::Butt => "butt".to_string(),
      StrokeCap::Round => "round".to_string(),
      StrokeCap::Square => "square".to_string(),
    }
  }
}

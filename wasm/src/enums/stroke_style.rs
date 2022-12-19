#[derive(Debug, Clone, Copy)]
pub enum StrokeStyle<'style_lifetime> {
  Rgb(u8, u8, u8),
  Rgba(u8, u8, u8, f64),
  CSSLiteral(String),
  Hex(u32),
}

impl ToString for StrokeStyle {
  fn to_string(&self) -> String {
    match &self {
      StrokeStyle::Rgb(red, green, blue) => {
        format!("rgb({red}, {green}, {blue})")
      }
      StrokeStyle::Rgba(red, green, blue, alpha) => {
        format!("rgba({red}, {green}, {blue}, {alpha})")
      }
      StrokeStyle::CSSLiteral(literal) => literal.to_owned(),
      StrokeStyle::Hex(val) => format!("#{:x}", val),
    }
  }
}

impl Default for StrokeStyle {
  fn default() -> Self {
    Self::Rgb(0, 0, 0)
  }
}

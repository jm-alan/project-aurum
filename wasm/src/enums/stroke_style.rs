#[derive(Debug, Clone, Copy)]
pub enum StrokeStyle<'style_lifetime> {
  Rgb(u8, u8, u8),
  Rgba(u8, u8, u8, f64),
  CSSLiteral(&'style_lifetime str),
  Hex(u32),
}

impl<'style_lifetime> ToString for StrokeStyle<'style_lifetime> {
  fn to_string(&self) -> String {
    match &self {
      StrokeStyle::Rgb(red, green, blue) => {
        format!("rgb({red}, {green}, {blue})")
      }
      StrokeStyle::Rgba(red, green, blue, alpha) => {
        format!("rgba({red}, {green}, {blue}, {alpha})")
      }
      StrokeStyle::CSSLiteral(literal) => (*literal).to_owned(),
      StrokeStyle::Hex(val) => format!("#{:x}", val),
    }
  }
}

impl<'style_lifetime> Default for StrokeStyle<'style_lifetime> {
  fn default() -> Self {
    Self::Rgb(0, 0, 0)
  }
}

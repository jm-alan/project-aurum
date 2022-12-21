use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub enum StrokeStyle {
  Rgb(u8, u8, u8),
  Rgba(u8, u8, u8, f64),
  Hsl(f64, f64, f64),
  Hsla(f64, f64, f64, f64),
  CSSLiteral(String),
  Hex(u32),
}

impl Default for StrokeStyle {
  fn default() -> Self {
    Self::Rgb(0, 0, 0)
  }
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
      StrokeStyle::Hsl(hue, saturation, lightness) => {
        format!("hsl({hue} {saturation}% {lightness}%)")
      }
      StrokeStyle::Hsla(hue, saturation, lightness, alpha) => {
        format!("hsla({hue} {saturation}% {lightness}% / {alpha})")
      }
      StrokeStyle::CSSLiteral(literal) => literal.to_owned(),
      StrokeStyle::Hex(val) => format!("#{:x}", val),
    }
  }
}

impl From<StrokeStyle> for JsValue {
  fn from(value: StrokeStyle) -> Self {
    value.to_string().into()
  }
}

impl From<&str> for StrokeStyle {
  fn from(value: &str) -> Self {
    Self::CSSLiteral(value.to_owned())
  }
}

impl From<(u8, u8, u8)> for StrokeStyle {
  fn from(value: (u8, u8, u8)) -> Self {
    let (red, green, blue) = value;
    Self::Rgb(red, green, blue)
  }
}

impl From<(u8, u8, u8, f64)> for StrokeStyle {
  fn from(value: (u8, u8, u8, f64)) -> Self {
    let (red, green, blue, alpha) = value;
    Self::Rgba(red, green, blue, alpha)
  }
}

impl From<u32> for StrokeStyle {
  fn from(value: u32) -> Self {
    Self::Hex(value)
  }
}

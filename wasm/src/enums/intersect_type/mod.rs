mod intersect_resolver_from_intersect_type;

use std::fmt::{Debug, Formatter, Result};

use crate::{
  canvas_point::CanvasPoint, types::intersect_resolver::IntersectResolver,
};

#[derive(Default)]
pub enum IntersectType {
  #[default]
  None,
  Triangle(CanvasPoint, CanvasPoint, CanvasPoint),
  Rhombus(CanvasPoint, CanvasPoint, CanvasPoint, CanvasPoint),
  Circle(CanvasPoint, f64),
  Ellipse(CanvasPoint, f64, f64, f64),
  Polygon(Vec<CanvasPoint>),
  Custom(IntersectResolver),
}

impl Debug for IntersectType {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
    match self {
      Self::Custom(_) => {
        formatter.write_fmt(format_args!("IntersectType::Custom"))
      }
      other => formatter.write_fmt(format_args!("{:?}", other)),
    }
  }
}

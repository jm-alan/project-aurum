mod intersect_resolver_from_intersect_type;

use crate::{
  canvas_point::CanvasPoint, types::intersect_resolver::IntersectResolver,
};

pub enum IntersectType {
  Triangle(CanvasPoint, CanvasPoint, CanvasPoint),
  Rectangle(CanvasPoint, CanvasPoint, CanvasPoint, CanvasPoint),
  Circle(CanvasPoint, f64),
  Ellipse(CanvasPoint, f64, f64),
  Polygon(Vec<CanvasPoint>),
  Custom(IntersectResolver),
}

impl Default for IntersectType {
  fn default() -> Self {
    Self::Polygon(vec![])
  }
}

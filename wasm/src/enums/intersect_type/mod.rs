mod intersect_resolver_from_intersect_type;

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

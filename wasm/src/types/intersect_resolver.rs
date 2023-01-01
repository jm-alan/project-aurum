use crate::canvas_point::CanvasPoint;

pub type IntersectResolver = Box<dyn Fn(CanvasPoint) -> bool>;

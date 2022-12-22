pub mod builder;
mod contiguous_shape_from_contiguous_shape_builder;

use web_sys::CanvasRenderingContext2d;

use crate::{
  canvas_point::CanvasPoint, enums::stroke_kind::StrokeKind,
  shape_config::ShapeConfig, shape_segment::ShapeSegment,
};

#[derive(Debug)]
pub struct ContiguousShape {
  pub start: CanvasPoint,
  pub closed_shape: bool,
  pub filled_shape: bool,
  pub segments: Vec<ShapeSegment>,
  pub config: ShapeConfig,
}

impl ContiguousShape {
  pub fn draw(self, context: &CanvasRenderingContext2d) {
    context.begin_path();
    context.set_stroke_style(&self.config.style.to_string().into());
    context.set_line_width(self.config.width.into());
    context.set_line_cap(&self.config.cap.to_string());
    context.move_to(self.start.x, self.start.y);
    for segment in self.segments.iter() {
      match segment.stroke_kind {
        StrokeKind::Line => {
          context.line_to(segment.coordinates.x, segment.coordinates.y);
        }
        StrokeKind::Arc(radius, control) => {
          let Ok(_) = context.arc_to(
            control.x,
            control.y,
            segment.coordinates.x,
            segment.coordinates.y,
            radius,
          ) else {
            console_log!("Invalid parameters provided to context.arc_to aka context.arcTo; panicking");
            panic!();
          };
        }
      }
    }
    if self.filled_shape {
      context.set_fill_style(&self.config.fill.to_string().into());
      context.fill();
    } else if self.closed_shape {
      context.close_path();
      context.stroke();
    } else {
      context.stroke();
    };
  }
}

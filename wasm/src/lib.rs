mod canvas_point;
mod contiguous_shape;
mod enums;
mod logging;
mod shape_config;
mod shape_segment;
mod stroke_batch;

use std::f64::consts::PI;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

use enums::{stroke_style::StrokeStyle, stroke_width::StrokeWidth};
use logging::console_log;
use shape_config::ShapeConfig;
use stroke_batch::StrokeBatch;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log(s: &str);
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console, js_name = error)]
  fn err(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
  let Some(window_ref) = window() else {
    js_panic!("Failed to grab window object from DOM");
  };

  let Some(document) = window_ref.document() else {
    js_panic!("Failed to grab document from window");
  };

  let Some(element) = document.get_element_by_id("main") else {
    js_panic!("Failed to grab canvas by ID");
  };

  let Ok(canvas) = element.dyn_into::<HtmlCanvasElement>() else {
    js_panic!("Failed to cast #main as canvas");
  };

  let Ok(Some(object)) = canvas.get_context("2d") else {
    js_panic!("Failed to get object representing render context");
  };

  let Ok(context) = object.dyn_into::<CanvasRenderingContext2d>() else {
    js_panic!("Failed to cast object as render context");
  };

  let mut batch = StrokeBatch::from(&context);

  batch
    .shape_from((0.0, 0.0).into())
    .config(ShapeConfig {
      fill: "white".into(),
      ..Default::default()
    })
    .line_through((f64::from(canvas.width()), 0.0).into())
    .line_through(
      (f64::from(canvas.width()), f64::from(canvas.height())).into(),
    )
    .line_through((0.0, f64::from(canvas.height())).into())
    .filled();

  let segments_float = f64::from(segments);
  let angle_offset = (PI * 2.0) / segments_float;
  let octagon_edge = (((radius * radius) * 2.0)
    - (2.0 * radius * radius * angle_offset.cos()))
  .sqrt();

  for i in 0..segments {
    let clamped_color_increment = 360.0 / segments_float;
    let iteration_float = f64::from(i);

    let next_angle = (angle_offset * iteration_float) + additional_offset;

    batch
      .shape_from((center_x, center_y).into())
      .config(ShapeConfig {
        style: StrokeStyle::Hsl(
          iteration_float * clamped_color_increment,
          100.0,
          50.0,
        ),
        width: StrokeWidth::Thin,
        ..Default::default()
      })
      .angle_through(next_angle, radius)
      .angle_through(
        next_angle + (PI - ((PI - ((PI * 2.0) / segments_float)) / 2.0)),
        octagon_edge,
      )
      .outlined();

    batch
      .shape_from((center_x, center_y).into())
      .config(ShapeConfig {
        fill: StrokeStyle::Hsl(
          iteration_float * clamped_color_increment,
          100.0,
          50.0,
        ),
        ..Default::default()
      })
      .angle_through(next_angle, radius)
      .angle_through(
        next_angle + (PI - ((PI - ((PI * 2.0) / segments_float)) / 2.0)),
        octagon_edge,
      )
      .filled();
  }
  batch.draw();
}

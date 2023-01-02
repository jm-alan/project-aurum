mod canvas_point;
mod contiguous_shape;
mod enums;
mod shape_config;
mod shape_segment;
mod stroke_batch;
mod types;

#[macro_use]
mod macros;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

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

  let square_count = 3;
  let square_count_float = f64::from(square_count);

  batch.circle((500.0, 500.0).into(), 100.0);

  for i in 0..square_count {
    batch.radial_square(
      (500.0, 500.0).into(),
      f64::from(i) * (2.0 / square_count_float) * std::f64::consts::PI,
      200.0,
    );
  }

  batch.draw();
}

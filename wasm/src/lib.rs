mod canvas_point;
mod contiguous_shape;
mod enums;
mod logging;
mod shape_segment;
mod stroke_batch;
mod stroke_config;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

use logging::console_log;
use stroke_batch::StrokeBatch;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
  let Some(window_ref) = window() else {
    console_log!("Failed to grab window object from DOM; panicking");
    panic!();
  };

  let Some(document) = window_ref.document() else {
    console_log!("Failed to grab document from window; panicking");
    panic!();
  };

  let Some(element) = document.get_element_by_id("main") else {
    console_log!("Failed to grab canvas by ID; panicking");
    panic!();
  };

  let Ok(canvas) = element.dyn_into::<HtmlCanvasElement>() else {
    console_log!("Failed to cast grabbed element as canvas; panicking");
    panic!();
  };

  let Ok(Some(object)) = canvas.get_context("2d") else {
    console_log!("Failed to get canvas rendering context; panicking");
    panic!();
  };

  let Ok(context) = object.dyn_into::<CanvasRenderingContext2d>() else {
    console_log!("Failed to cast object as render context; panicking");
    panic!();
  };

  let batch: StrokeBatch = (&context).into();
}

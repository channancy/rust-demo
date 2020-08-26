mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rand::prelude::*;

#[wasm_bindgen]
pub fn draw(element_id: String) {
    utils::set_panic_hook();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas_target = format!("{}", element_id);
    let canvas = document.get_element_by_id(&canvas_target).unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut rng = thread_rng();
    let r = rng.gen_range(0, 256);
    let g = rng.gen_range(0, 256);
    let b = rng.gen_range(0, 256);
    let rgb_string = format!("rgb({},{},{})", r, g, b);

    context.set_fill_style(&rgb_string.into());   
    context.fill_rect(15.0, 15.0, 80.0, 80.0);
}
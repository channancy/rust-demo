mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rand::Rng;

#[wasm_bindgen]
pub fn draw(position: i32) {
    utils::set_panic_hook();
    
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0, 256);
    let g = rng.gen_range(0, 256);
    let b = rng.gen_range(0, 256);

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas_target = format!("canvas{}", position.to_string());
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

    let rgb = format!("rgb({},{},{})", r.to_string(), g.to_string(), b.to_string());
    context.set_fill_style(&rgb.into());    
    context.fill_rect(15.0, 15.0, 80.0, 80.0);
}
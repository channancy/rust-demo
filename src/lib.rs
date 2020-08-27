mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rand::Rng;

#[wasm_bindgen]
pub struct Shape {
    r: u32,
    g: u32,
    b: u32
}

#[wasm_bindgen]
impl Shape {
    pub fn draw(&self, element_id: String) {
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

        let rgb = format!("rgb({},{},{})", self.r.to_string(), self.g.to_string(), self.b.to_string());
        context.set_fill_style(&rgb.into());    
        context.fill_rect(15.0, 15.0, 80.0, 80.0);
    }

    pub fn new() -> Shape {
        utils::set_panic_hook();

        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0, 256);
        let g = rng.gen_range(0, 256);
        let b = rng.gen_range(0, 256);

        Shape {
            r: r,
            g: g,
            b: b
        }
    }
}

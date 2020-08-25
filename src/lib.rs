mod utils;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize)]
struct Colors {
    colors: Vec<Color>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub hex: String
}

#[wasm_bindgen]
pub async fn get_colors() -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("http://www.colr.org/json/colors/random/6");
    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Use serde to parse the JSON into a struct.
    let color_info: Colors = json.into_serde().unwrap();

    // Send the struct back to JS as an `Object`.
    Ok(JsValue::from_serde(&color_info).unwrap())
}

#[wasm_bindgen]
pub fn draw(color: String, position: i32) {
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

    let hex_color = format!("#{}", color);
    context.set_fill_style(&hex_color.into());    
    context.fill_rect(15.0, 15.0, 80.0, 80.0);
}
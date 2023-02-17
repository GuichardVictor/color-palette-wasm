mod color;
mod palette;

extern crate wasm_bindgen;
extern crate js_sys;

use wasm_bindgen::{prelude::*};

#[wasm_bindgen]
pub fn extract_color_palette(image_data: &[u8]) -> js_sys::Uint8Array {
    let color_palette: Vec<u8> = palette::extract_color_palette(&image_data).iter().flat_map(|x| x.iter()).cloned().collect();
    js_sys::Uint8Array::from(&color_palette[..])
}

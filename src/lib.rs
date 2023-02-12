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

#[cfg(test)]
mod tests {
    use super::*;
    use image::imageops::FilterType::Nearest;
    use image::io::Reader as ImageReader;
    use std::time::Instant;

    #[test]
    fn it_works() {
        let img = ImageReader::open("examples/Marc_Chagall_202.jpg")
            .unwrap()
            .decode()
            .unwrap();
        let img = img.resize(400, 320, Nearest);

        let now = Instant::now();
        let palette = palette::extract_color_palette(&img.to_rgb8());
        let elapsed = now.elapsed();

        println!("{:?}", palette);

        println!("Elapsed: {:.2?}", elapsed);
    }

    #[test]
    fn test_color_conversion() {
        let rgb = [128, 30, 156];
        assert_eq!(rgb, color::lab2rgb(&color::rgb2lab(&rgb)));
    }
}

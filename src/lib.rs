extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use image::*;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn time(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn timeEnd(s: &str);
}

#[wasm_bindgen]
pub fn wasm_run() {}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn resize_image(
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    width: usize,
    height: usize,
    fmt: &str,
) -> Uint8Array {
    console_error_panic_hook::set_once();

    time("get_image_data in Rust");
    time("replace image::load_from_memory() in Rust");
    let image_width = canvas.width();
    let image_height = canvas.height();

    let data: ImageData = ctx
        .get_image_data(0.0, 0.0, image_width as f64, image_height as f64)
        .unwrap();
    timeEnd("get_image_data in Rust");
    time("data.data() in Rust");
    let image_data = data.data().to_vec();
    timeEnd("data.data() in Rust");

    // バッファから画像を読み込む
    time("ImageBuffer::from_vec in Rust");
    let img_buffer = ImageBuffer::from_vec(image_width, image_height, image_data).unwrap();
    timeEnd("ImageBuffer::from_vec in Rust");
    time("DynamicImage::ImageRgba8 in Rust");
    let dynamic_image = DynamicImage::ImageRgba8(img_buffer);
    timeEnd("DynamicImage::ImageRgba8 in Rust");
    timeEnd("replace image::load_from_memory() in Rust");

    // 指定サイズに画像をリサイズする
    time("image::imageops::resize() in Rust");
    let resized = DynamicImage::ImageRgba8(image::imageops::resize(
        &dynamic_image,
        width as u32,
        height as u32,
        imageops::FilterType::Nearest,
    ));
    timeEnd("image::imageops::resize() in Rust");

    // バッファに画像を書き出す
    time("save_to_buffer in Rust");
    let result = save_to_buffer(resized, fmt);
    timeEnd("save_to_buffer in Rust");

    // バッファから Uint8Array を作成
    time("Vec<u8> to Uint8Array in Rust");
    let resp = Uint8Array::new(&unsafe { Uint8Array::view(&result) }.into());
    timeEnd("Vec<u8> to Uint8Array in Rust");

    resp
}

// バッファに画像を書き出す
fn save_to_buffer(img: DynamicImage, fmt_str: &str) -> Vec<u8> {
    console_error_panic_hook::set_once();

    let fmt = match fmt_str {
        "png" => ImageOutputFormat::Png,
        "gif" => ImageOutputFormat::Gif,
        "bmp" => ImageOutputFormat::Bmp,
        "jpg" => ImageOutputFormat::Jpeg(80),
        unsupport => ImageOutputFormat::Unsupported(String::from(unsupport)),
    };

    // バッファを確保して画像を書き出す
    let mut result: Vec<u8> = Vec::new();
    img.write_to(&mut result, fmt)
        .expect("Error occurs at save image from buffer.");

    result
}

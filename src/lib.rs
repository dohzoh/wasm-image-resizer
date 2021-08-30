extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use image::*;
use js_sys::*;
use photon_rs::transform::SamplingFilter;
use photon_rs::*;
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
pub fn wasm_run() {
    let _res = photon_rs::run();
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn sum_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

#[wasm_bindgen]
pub fn resize_image(
    canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    width: usize,
    height: usize,
    fmt: &str,
) -> Uint8Array {
    console_error_panic_hook::set_once();

    time("get_image_data in Rust");
    let image_width = canvas.width();
    let image_height = canvas.height();

    // let data: ImageData = ctx.get_image_data(0.0, 0.0, 100.0, 100.0).unwrap();
    let data: ImageData = ctx
        .get_image_data(0.0, 0.0, image_width as f64, image_height as f64)
        .unwrap();
    timeEnd("get_image_data in Rust");
    time("data.data() in Rust");
    let image_data = data.data().to_vec();
    //    let photon_image = photon_rs::open_image(canvas, ctx);
    timeEnd("data.data() in Rust");

    // バッファから画像を読み込む
    time("ImageBuffer::from_vec in Rust");
    //    let _len_vec = photon_image.raw_pixels.len() as u128;
    let img_buffer = image::ImageBuffer::from_vec(image_width, image_height, image_data).unwrap();
    timeEnd("ImageBuffer::from_vec in Rust");
    time("DynamicImage::ImageRgba8 in Rust");
    let dynamic_image = DynamicImage::ImageRgba8(img_buffer);

    //    let dynamic_image = helpers::dyn_image_from_raw(&photon_image);
    timeEnd("DynamicImage::ImageRgba8 in Rust");

    // 指定サイズに画像をリサイズする
    time("image::resize_exact() in Rust");
    let resized =
        dynamic_image.resize_exact(width as u32, height as u32, imageops::FilterType::Triangle);
    timeEnd("image::resize_exact() in Rust");

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

#[wasm_bindgen]
pub fn resize_image_browser(
    canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    width: u32,
    height: u32,
    sampling_filter: SamplingFilter,
) -> web_sys::HtmlCanvasElement {
    console_error_panic_hook::set_once();

    time("photon_rs::open_image in Rust");
    let photon_image = photon_rs::open_image(canvas, ctx);
    timeEnd("photon_rs::open_image in Rust");

    // バッファから画像を読み込む
    time("photon_rs::transform::resize_img_browser in Rust");
    let canvas =
        photon_rs::transform::resize_img_browser(&photon_image, width, height, sampling_filter);
    timeEnd("photon_rs::transform::resize_img_browser in Rust");

    canvas
}

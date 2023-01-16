use init_data::calc_rt_bounds::PixelCoods;
use pixel_processing::scan_line::scan_line;
use wasm_bindgen::prelude::*;

use crate::init_data::calc_rt_bounds::PxData;
pub mod init_data;
pub mod output_json;
pub mod pixel_processing;
pub mod volume_tools;

// #[wasm_bindgen]
// pub fn fib(n: Vec<f64>) -> Vec<f64> {
//     let rt = JsValue::from(n);
//     let line_result = scan_line(n);
//     return line_result;
// }

#[wasm_bindgen]
pub fn fib() -> i32 {
    let mut result = Vec::new();
    for i in 0..1000000 {
        result.push(i * 2)
    }

    return result[1];
}

use init_data::calc_rt_bounds::{get_rt_pxdata_and_bounds, get_volume_bounds};
use init_data::init_json::ImageInfo;
use pixel_processing::build_xy_rt::{generate_mask, mask_to_rt};
// use pixel_processing::magic_wand::Contours;
use pixel_processing::scan_line::scan_line;
// use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
extern crate serde_json;

pub mod init_data;
pub mod output_json;
pub mod pixel_processing;
pub mod volume_tools;


// #[wasm_bindgen]
// extern "C" {
//     // pub type ImageInfo;
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn rt2rt(val: JsValue) -> JsValue {
    let params: ImageInfo = serde_wasm_bindgen::from_value(val).unwrap();
    // 获取体数据的边界
    let volume_bounds = get_volume_bounds(&params);
    // 物理坐标转像素坐标，并寻找边界
    let rt_pxdata_and_bounds = get_rt_pxdata_and_bounds(&params);
    // 扫描线算法
    let line_result = scan_line(rt_pxdata_and_bounds);
    // 生成切面mask轮廓
    let rt_build_mask = generate_mask(line_result, &volume_bounds);
    // 轮廓提取
    let rt_build_result = mask_to_rt(rt_build_mask, &volume_bounds);
    serde_wasm_bindgen::to_value(&rt_build_result).unwrap()
}

// #[wasm_bindgen]
// pub fn rt2rt(numbers: Box<[JsValue]>) -> Box<[JsValue]> {
//     // vec![JsValue::NULL, JsValue::UNDEFINED, JsValue::].into_boxed_slice();
//     for _value in numbers.iter() {
//         // alert(&_value);
//     }
//     let mut result = Vec::new();
//     for (i, v) in numbers.iter().enumerate() {
//         if v.is_object() {
//             // for (index, value) in v.iter() {

//             // }
//         }
//         if !v.is_undefined() {
//             result.push(Some(v.as_f64().unwrap() as u8).unwrap().into());
//         }
//     }

//     result.into_boxed_slice()
//     // vec![
//     //     "Hello".into(),
//     //     512.into(),
//     //     JsValue::NULL,
//     //     JsValue::UNDEFINED,
//     //     61.20.into(),
//     // ]
//     // .into_boxed_slice()
// }


use init_data::calc_rt_bounds::get_rt_pxdata_and_bounds;
use init_data::init_json::ImageInfo;
use pixel_processing::{
    build_xy_rt::{generate_mask, mask_to_rt},
};
// use pixel_processing::magic_wand::Contours;
use pixel_processing::scan_line::scan_line;
// use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
extern crate serde_json;
use js_sys::{Int8Array, Object, Reflect};
pub mod init_data;
pub mod output_json;
pub mod pixel_processing;

#[wasm_bindgen]
pub fn rt2rt(val: JsValue) -> Result<JsValue, JsError> {
    let params: ImageInfo = serde_wasm_bindgen::from_value(val).unwrap();
    // 物理坐标转像素坐标，并寻找边界
    let rt_pxdata_and_bounds = get_rt_pxdata_and_bounds(&params);
    // 扫描线算法
    let mask_volume = scan_line(rt_pxdata_and_bounds, &params);
    // 生成切面mask轮廓
    let rt_build_mask = generate_mask(&mask_volume, &params);
    // 轮廓提取
    let rt_build_result = mask_to_rt(rt_build_mask, &params);

    let return_volume = params.return_volume;

    // 创建一个 JavaScript 对象
    let obj = Object::new();

    let volume = if return_volume {
        let array = Int8Array::new_with_length(mask_volume.len() as u32);
        array.copy_from(&mask_volume);
        array
    } else {
        let array = Int8Array::new_with_length(0);
        array
    };

    // 将 ArrayBuffer 添加到对象中
    Reflect::set(&obj, &JsValue::from_str("volume"), &volume.into()).unwrap();
    Reflect::set(
        &obj,
        &JsValue::from_str("x"),
        &serde_wasm_bindgen::to_value(&rt_build_result.x).unwrap(),
    )
    .unwrap();
    Reflect::set(
        &obj,
        &JsValue::from_str("y"),
        &serde_wasm_bindgen::to_value(&rt_build_result.y).unwrap(),
    )
    .unwrap();

    Ok(obj.into())
    // result
    // serde_wasm_bindgen::to_value(&result).unwrap()
}

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


#[wasm_bindgen]
pub fn rt2rt(val: JsValue) -> JsValue {
    let params: ImageInfo = serde_wasm_bindgen::from_value(val).unwrap();
    // 获取体数据的边界
    let volume_bounds = get_volume_bounds(&params);
    // 物理坐标转像素坐标，并寻找边界
    let rt_pxdata_and_bounds = get_rt_pxdata_and_bounds(&params, &volume_bounds);
    // 扫描线算法
    let mask_volume = scan_line(rt_pxdata_and_bounds, &volume_bounds);
    // 生成切面mask轮廓
    let rt_build_mask = generate_mask(mask_volume, &volume_bounds);
    // 轮廓提取
    let rt_build_result = mask_to_rt(rt_build_mask, &volume_bounds);
    serde_wasm_bindgen::to_value(&rt_build_result).unwrap()
}



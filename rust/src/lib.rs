use serde::{Deserialize, Serialize};
// use init_data::init_json::ImageInfo;
// use init_data::calc_rt_bounds::PixelCoods;
// use pixel_processing::scan_line::scan_line;
use wasm_bindgen::prelude::*;
extern crate serde_json;
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// use crate::init_data::calc_rt_bounds::PxData;
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
// #[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ObjArray {
    pub data: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ImageInfo {
    pub column: u32,
    pub row: u32,
    pub lay_num: u32,
    pub row_pixel_spacing: f64,
    pub column_pixel_spacing: f64,
    pub thickness: f64,
    pub zm_obj_array: ObjArray,
    pub zm_data: Vec<Vec<u32>>,
    pub zm_data_obj: Vec<Person>,
    // pub image_position_patient: Vec<f64>,
    // pub data: Box<Vec<Vec<f64>>>,
}

// #[macro_use]
// extern crate serde_derive;

#[wasm_bindgen]
extern "C" {
    // pub type ImageInfo;
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn getObj (val: JsValue) -> JsValue {
    let mut res: ImageInfo = serde_wasm_bindgen::from_value(val).unwrap();
    let firstData: u32 = res.zm_obj_array.data[0] * 2;
    let mut person = Person {
        name: String::from("FEA_Dven"),
        age: 20,
    };
    let new_array = vec![1, 2, 3];
    let two_array = vec![4, 5, 6];
    let mut i = 0;
    while i < 2 {
        let person = Person {
            name: String::from("FEA_Dven"),
            age: 20 + i,
        };
        res.zm_data_obj.push(person);
        i += 1
    }
    res.zm_obj_array.data.push(firstData);
    res.zm_data.push(new_array);
    res.zm_data.push(two_array);
    serde_wasm_bindgen::to_value(&res).unwrap()
}

#[wasm_bindgen]
pub fn rt2rt(numbers: Box<[JsValue]>) -> Box<[JsValue]>{
    // vec![JsValue::NULL, JsValue::UNDEFINED, JsValue::].into_boxed_slice();
    for _value in numbers.iter() {
        // alert(&_value);
    }
    let mut result = Vec::new();
    for (i, v) in numbers.iter().enumerate() {
        if v.is_object() {
            // for (index, value) in v.iter() {
                
            // }
        }
        if !v.is_undefined() {
            result.push(Some(v.as_f64().unwrap() as u8).unwrap().into());
        }
    }

    result.into_boxed_slice()
    // vec![
    //     "Hello".into(),
    //     512.into(),
    //     JsValue::NULL,
    //     JsValue::UNDEFINED,
    //     61.20.into(),
    // ]
    // .into_boxed_slice()
}

// #[wasm_bindgen]
// pub fn fib() -> i32 {
//     let mut result = Vec::new();
//     for i in 0..1000000 {
//         result.push(i * 2)
//     }

//     return result[1];
// }

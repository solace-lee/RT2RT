extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageInfo {
    pub column: u32, // x轴像素
    pub row: u32, // y轴像素
    #[serde(rename = "layNum")]
    pub lay_num: u32, // z轴像素
    #[serde(rename = "rowPixelSpacing")]
    pub row_pixel_spacing: f64, // 行间距
    #[serde(rename = "columnPixelSpacing")]
    pub column_pixel_spacing: f64, // 列间距
    pub thickness: f64, // 切片厚度
    #[serde(rename = "imagePositionMatrix")]
    pub image_position_matrix: Vec<[f32; 16]>,
    #[serde(rename = "pixelSpacingNormalized")]
    pub pixel_spacing_normalized: f64, // Z轴的拉伸倍率
    #[serde(rename = "xLayer")]
    pub x_layer: f64, // x轴 像素/层thickness / row_pixel_spacing
    #[serde(rename = "yLayer")]
    pub y_layer: f64, // y轴 像素/ 层thickness / column_pixel_spacing
    // #[serde(flatten)]
    #[serde(rename = "returnVolume")]
    pub return_volume: bool, // 是否返回体积数据
    pub data: Vec<Vec<Vec<f32>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cood {
    pub x: f32,
    pub y: f32,
}

impl ImageInfo {
    pub fn new(path: &str) -> Result<ImageInfo> {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        let v: ImageInfo = serde_json::from_reader(reader)?;
        Ok(v)
    }
}

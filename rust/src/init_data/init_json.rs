extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageInfo {
    pub column: u32,
    pub row: u32,
    #[serde(rename = "layNum")]
    pub lay_num: u32,
    #[serde(rename = "rowPixelSpacing")]
    pub row_pixel_spacing: f64,
    #[serde(rename = "columnPixelSpacing")]
    pub column_pixel_spacing: f64,
    pub thickness: f64,
    // #[serde(flatten)]
    pub data: Vec<Vec<Vec<Cood>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cood {
    pub x: f64,
    pub y: f64,
}

impl ImageInfo {
    pub fn new(path: &str) -> Result<ImageInfo> {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        let v: ImageInfo = serde_json::from_reader(reader)?;
        Ok(v)
    }
}

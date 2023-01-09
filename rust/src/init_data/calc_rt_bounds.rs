use serde::{Deserialize, Serialize};

use crate::{init_data::init_json::ImageInfo, volume_tools::volume::volume::Bounds};

/// 轮廓像素坐标点结构定义
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PixelCoods {
    pub x: i32,
    pub y: i32,
}

/// 轮廓像素坐标集合及边界
#[derive(Debug, Serialize, Deserialize)]
pub struct PxData {
    pub data: Vec<Vec<Vec<PixelCoods>>>,
    pub bounds: BoundsLimit,
    pub layer_bounds: Vec<BoundsLimit>,
}

///轮廓像素边界定义
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct BoundsLimit {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

///寻找XYZ的最小像素间距
// pub fn find_pixel_spacing(arr: Vec<f64>) -> f64 {
//     let min = arr.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
//     *min
// }

///计算volume的边界
pub fn get_volume_bounds(imagainfo: &ImageInfo) -> Bounds {
    let ImageInfo {
        column,
        row,
        lay_num,
        thickness,
        row_pixel_spacing,
        column_pixel_spacing,
        image_position_patient,
        ..
    } = imagainfo;
    Bounds {
        x: *column, // x轴像素
        y: *row, // y轴像素
        z: *lay_num, // z轴像素
        x_layer: thickness / row_pixel_spacing, // x轴 像素/层
        y_layer: thickness / column_pixel_spacing, // y轴 像素/ 层
    }
}

///物理坐标转像素坐标并计算轮廓边界
pub fn get_rt_pxdata_and_bounds(imagainfo: &ImageInfo) -> PxData {
    let ImageInfo {
        data,
        row_pixel_spacing,
        column_pixel_spacing,
        ..
    } = imagainfo;

    // let result = data.iter().iter().iter().for_each(|v| )

    let mut result = Vec::new();
    let mut bounds = BoundsLimit {
        min_x: 0,
        max_x: 0,
        min_y: 0,
        max_y: 0,
    };

    let mut layer_bounds: Vec<BoundsLimit> = Vec::new();

    for index in 0..data.len() {
        // for index in 0..2 {
        let i = &data[index];
        let mut o = Vec::new();

        // 单层的轮廓范围
        let mut item_bounds = BoundsLimit {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        };
        if i.len() != 0 {
            for j in i {
                let mut p = Vec::new();
                if j.len() != 0 {
                    for k in 0..(j.len() / 2) {
                        let kx = j[k * 2];
                        let ky = j[k * 2 + 1];
                        let x = (kx / row_pixel_spacing).ceil() as i32;
                        let y = (ky / column_pixel_spacing).ceil() as i32;

                        if item_bounds.max_x < x {
                            item_bounds.max_x = x
                        } else if item_bounds.min_x > x {
                            item_bounds.min_x = x
                        }
                        if item_bounds.max_y < y {
                            item_bounds.max_y = y
                        } else if item_bounds.min_y > y {
                            item_bounds.min_y = y
                        }

                        p.push(PixelCoods { x, y });
                        // p.push(x);
                        // p.push(y);
                    }
                };
                o.push(p);
            }
        };
        // 存储单层的轮廓范围
        layer_bounds.push(item_bounds);

        if bounds.max_x < item_bounds.max_x {
            bounds.max_x = item_bounds.max_x
        } else if bounds.min_x > item_bounds.min_x {
            bounds.min_x = item_bounds.min_x
        }
        if bounds.max_y < item_bounds.max_y {
            bounds.max_y = item_bounds.max_y
        } else if bounds.min_y > item_bounds.min_y {
            bounds.min_y = item_bounds.min_y
        }
        result.push(o);
    }

    PxData {
        data: result,
        bounds,
        layer_bounds,
    }
}

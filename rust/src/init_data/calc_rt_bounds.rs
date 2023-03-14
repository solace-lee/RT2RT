use serde::{Deserialize, Serialize};

use crate::init_data::init_json::ImageInfo;

#[derive(Clone, Debug)]
pub struct Bounds {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    // pub z_pixel_spacing: f32,
    pub x_layer: f64,
    pub y_layer: f64,
    pub px_position_patient: Vec<i64>,
}

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

    // 用于存储图像原点的像素坐标
    let mut px_position_patient = Vec::new();

    for i in 0..(image_position_patient.len() / 3) {
        let x = image_position_patient[i * 3];
        let y = image_position_patient[i * 3 + 1];
        let z = image_position_patient[i * 3 + 2];
        px_position_patient.push((x / row_pixel_spacing).ceil() as i64);
        px_position_patient.push((y / column_pixel_spacing).ceil() as i64);
        px_position_patient.push(z.ceil() as i64);
    }
    Bounds {
        x: *column,                                // x轴像素
        y: *row,                                   // y轴像素
        z: *lay_num,                               // z轴像素
        x_layer: thickness / row_pixel_spacing,    // x轴 像素/层
        y_layer: thickness / column_pixel_spacing, // y轴 像素/ 层
        px_position_patient,                       // 图像原点
    }
}

///物理坐标转像素坐标并计算轮廓边界
pub fn get_rt_pxdata_and_bounds(imagainfo: &ImageInfo, bounds: &Bounds) -> PxData {
    let ImageInfo {
        data,
        row_pixel_spacing,
        column_pixel_spacing,
        column,
        row,
        ..
    } = imagainfo;

    let Bounds {
        px_position_patient,
        ..
    } = bounds;

    let mut result = Vec::new();
    let mut bounds = BoundsLimit {
        min_x: 0,
        max_x: 0,
        min_y: 0,
        max_y: 0,
    };

    let mut layer_bounds: Vec<BoundsLimit> = Vec::new();

    let z_position_layer_num = px_position_patient.len() / 3;
    let max_colume = *column as i32;
    let max_row = *row as i32;

    for index in 0..data.len() { // 遍历每一层
        let position_index = (index + z_position_layer_num) % z_position_layer_num;
        let px_position_x = px_position_patient[position_index * 3] as i32;
        let px_position_y = px_position_patient[position_index * 3 + 1] as i32;
        
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
                        let x = (kx / row_pixel_spacing).ceil() as i32 - px_position_x;
                        let y = (ky / column_pixel_spacing).ceil() as i32 - px_position_y;

                        if x < 0 || y < 0 || x > max_colume || y > max_row {
                            // 剔除超过dicom范围的坐标数据
                            continue;
                        }

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

use glam::{Mat4, Vec4};
use serde::{Deserialize, Serialize};

use crate::init_data::init_json::ImageInfo;

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

///物理坐标转像素坐标并计算轮廓边界
pub fn get_rt_pxdata_and_bounds(imagainfo: &ImageInfo) -> PxData {
    let ImageInfo {
        data,
        column,
        row,
        image_position_matrix,
        ..
    } = imagainfo;

    let mut result = Vec::new();
    let mut bounds = BoundsLimit {
        min_x: 0,
        max_x: 0,
        min_y: 0,
        max_y: 0,
    };

    let mut layer_bounds: Vec<BoundsLimit> = Vec::new();

    let matrix_length = image_position_matrix.len();
    let max_colume = *column as i32;
    let max_row = *row as i32;

    for (index, item) in data.iter().enumerate() {
        // 遍历每一层
        let position_index = (index + matrix_length) % matrix_length;

        let i = item;
        let mut o = Vec::new();

        // 单层的轮廓范围
        let mut item_bounds = BoundsLimit {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        };
        if !i.is_empty() {
            for j in i {
                let mut p = Vec::new();
                if !j.is_empty() {
                    for k in 0..(j.len() / 2) {
                        let kx = j[k * 2];
                        let ky = j[k * 2 + 1];
                        let py_point = Vec4::new(kx, ky, 0.0, 1.0);
                        let matrix =
                            Mat4::from_cols_array(&(image_position_matrix[position_index]));
                        let i_point = matrix.mul_vec4(py_point);
                        let x = i_point.x as i32;
                        let y = i_point.y as i32;

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

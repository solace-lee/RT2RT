use serde::{Deserialize, Serialize};

use crate::init_data::init_json::ImageInfo;

use super::magic_wand::{Contours, Mask, trace_contours};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct RTMask {
    pub x_rt: Vec<Vec<isize>>,
    pub y_rt: Vec<Vec<isize>>,
    pub x_bounds: Vec<MaskBounds>,
    pub y_bounds: Vec<MaskBounds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct MaskBounds {
    pub minx: isize,
    pub miny: isize,
    pub maxx: isize,
    pub maxy: isize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RTContours {
    pub x: Vec<Vec<Contours>>,
    pub y: Vec<Vec<Contours>>,
}

// 基于线数据构建层mask
pub fn generate_mask(mask_volume: Vec<i8>, bounds: &ImageInfo) -> RTMask {
    let ImageInfo {
        column,
        row,
        lay_num,
        x_layer, // x轴 像素/层
        y_layer, // y轴 像素/ 层
        ..
    } = bounds;

    let x_layer_num = (*column as f64 / x_layer).ceil(); // 计算X切面的层数
    let y_layer_num = (*row as f64 / y_layer).ceil(); // 计算Y切面的层数

    // 初始化mask
    let mut result = RTMask {
        x_rt: vec![vec![0; (row * (lay_num + 1)) as usize]; x_layer_num as usize],
        y_rt: vec![vec![0; (column * (lay_num + 1)) as usize]; y_layer_num as usize],
        x_bounds: vec![
            MaskBounds {
                minx: *row as isize,
                miny: *lay_num as isize,
                maxx: 0,
                maxy: 0,
            };
            x_layer_num as usize
        ],
        y_bounds: vec![
            MaskBounds {
                minx: *column as isize,
                miny: *lay_num as isize,
                maxx: 0,
                maxy: 0,
            };
            y_layer_num as usize
        ],
    };

    for z_num in 0..*lay_num {
        let z_lay = z_num * column * row;
        for y_num in 0..*row {
            let current_y_layer = (y_num as f64 / y_layer).ceil();
            let y_lay = y_num * column;
            for x_num in 0..*column {
                let index = z_lay + y_lay + x_num;
                let value = mask_volume[index as usize];
                if value == 0 {
                    continue;
                }
                // 生成x切面
                let current_x_layer = (x_num as f64 / x_layer).ceil();
                let is_x_true = current_x_layer * x_layer == x_num as f64;
                if is_x_true {
                    let x_bounds = &mut result.x_bounds[current_x_layer as usize];
                    if x_bounds.minx > y_num as isize {
                        x_bounds.minx = y_num as isize;
                    }
                    if x_bounds.maxx < y_num as isize {
                        x_bounds.maxx = y_num as isize;
                    }
                    if x_bounds.miny > z_num as isize {
                        x_bounds.miny = z_num as isize;
                    }
                    if x_bounds.maxy < z_num as isize {
                        x_bounds.maxy = z_num as isize;
                    }
                    result.x_rt[current_x_layer as usize][(z_num * *row + y_num) as usize] = 1;
                }

                // 生成y切面
                let is_y_true = current_y_layer * y_layer == y_num as f64;
                if is_y_true {
                    let y_bounds = &mut result.y_bounds[current_y_layer as usize];
                    if y_bounds.minx > x_num as isize {
                        y_bounds.minx = x_num as isize;
                    }
                    if y_bounds.maxx < x_num as isize {
                        y_bounds.maxx = x_num as isize;
                    }
                    if y_bounds.miny > z_num as isize {
                        y_bounds.miny = z_num as isize;
                    }
                    if y_bounds.maxy < z_num as isize {
                        y_bounds.maxy = z_num as isize;
                    }
                    result.y_rt[current_y_layer as usize][(z_num * *column + x_num) as usize] = 1;
                }
            }
        }
    }
    result
}

pub fn mask_to_rt(all_mask: RTMask, image_info: &ImageInfo) -> RTContours {
    let ImageInfo {
        column,
        row,
        lay_num,
        pixel_spacing_normalized,
        ..
    } = image_info;

    let RTMask {
        x_rt,
        y_rt,
        x_bounds,
        y_bounds,
    } = all_mask;

    let mut result_data = RTContours {
        x: Vec::new(),
        y: Vec::new(),
    };

    // X截面
    for index in 0..x_rt.len() {
        result_data.x.push(Vec::new());
        let MaskBounds {
            minx,
            miny,
            maxx,
            maxy,
        } = x_bounds[index];
        if minx > maxx {
            continue;
        }
        let mask_item = &x_rt[index];

        // 提取mask的轮廓
        let mut contours = trace_contours(Mask {
            data: mask_item,
            width: *row as isize,
            height: *lay_num as isize,
            minx,
            miny,
            maxx,
            maxy: maxy + 1,
        });
        // if index == 64 {
        //     println!("hh, {:#?}, {}, {}", contours, minx, maxx);
        //     output::output(&contours, "./json/xxx.json");
        // }

        for item in &mut contours {
            // 轮廓数
            let length = item.points.len();
            for i in 0..(length / 2) {
                // 遍历每个轮廓
                let y = i * 2;
                // let z = item.points[y + 1];
                // let dy =
                //     px_position_patient[((layer_num + z as usize) % layer_num) * 3 + 1] as isize;
                const DY: isize = 0; // todo 后面直接用像素坐标
                item.points[y] += DY;
            }
        }
        result_data.x[index] = contours;
    }

    // Y截面
    for index in 0..y_rt.len() {
        result_data.y.push(Vec::new());
        let MaskBounds {
            minx,
            miny,
            maxx,
            maxy,
        } = y_bounds[index];
        if minx > maxx {
            continue;
        }
        let mask_item = &y_rt[index];

        // 提取mask的轮廓
        let mut contours = trace_contours(Mask {
            data: mask_item,
            width: *column as isize,
            height: *lay_num as isize,
            minx,
            miny,
            maxx,
            maxy: maxy + 1,
        });
        for item in &mut contours {
            // 轮廓数
            let length = item.points.len();
            for i in 0..(length / 2) {
                // 遍历每个轮廓
                let x = i * 2;
                let z = item.points[x + 1];
                // let dx = px_position_patient[(layer_num + z as usize) % layer_num * 3] as isize;
                const DX: isize = 0; // todo 后面直接用像素坐标
                item.points[x] += DX;
            }
        }
        result_data.y[index] = contours;
    }
    result_data
}

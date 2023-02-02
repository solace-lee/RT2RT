use serde::{Deserialize, Serialize};

use crate::init_data::calc_rt_bounds::Bounds;

use super::magic_wand::{trace_contours, Mask, Contours};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct RTMask {
    pub x_rt: Vec<Vec<isize>>,
    pub y_rt: Vec<Vec<isize>>,
    pub x_bounds: Vec<MaskBounds>,
    pub y_bounds: Vec<MaskBounds>,
    // pub x_width: u32,
    // pub y_width: u32,
    // pub x_height: u32,
    // pub y_height: u32,
    // pub px_position_patient: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct MaskBounds {
    pub minx: isize,
    pub miny: isize,
    pub maxx: isize,
    pub maxy: isize,
}

// 构建
pub fn generate_mask(line: Vec<Vec<i32>>, bounds: &Bounds) -> RTMask {
    let Bounds {
        x,
        y,
        z,
        x_layer,             // x轴 像素/层
        y_layer,             // y轴 像素/ 层
        px_position_patient, // 每层的原点像素坐标
    } = bounds;
    let x_layer_num = (*x as f64 / x_layer.round()).ceil(); // 计算X切面的层数
    let y_layer_num = (*y as f64 / y_layer.round()).ceil(); // 计算Y切面的层数

    // 初始化mask
    let mut result = RTMask {
        x_rt: vec![vec![0; (y * z) as usize]; x_layer_num as usize],
        y_rt: vec![vec![0; (x * z) as usize]; y_layer_num as usize],
        x_bounds: vec![
            MaskBounds {
                minx: *y as isize,
                miny: *z as isize,
                maxx: 0,
                maxy: 0,
            };
            x_layer_num as usize
        ],
        y_bounds: vec![
            MaskBounds {
                minx: *x as isize,
                miny: *z as isize,
                maxx: 0,
                maxy: 0,
            };
            y_layer_num as usize
        ],
    };

    // println!(
    //     "X轴层数{:?},像素{:?} Y轴层数{:?}, 像素{:?}",
    //     result.x_rt.len(),
    //     x_layer.round(),
    //     result.y_rt.len(),
    //     y_layer.round(),
    // );

    for z in 0..line.len() {
        let layer_coords = &line[z]; // z层数
        let position_index = z % (px_position_patient.len() / 3);
        let px_position_x = px_position_patient[position_index] as isize;
        let px_position_y = px_position_patient[position_index + 1] as isize;

        // println!("x起始像素{}，y起始像素{}", px_position_x, px_position_y);

        for line_index in 0..(layer_coords.len() / 3) as usize {
            let line_x_start = &layer_coords[line_index * 3];
            let line_x_end = &layer_coords[line_index * 3 + 1];
            let line_y = &layer_coords[line_index * 3 + 2];
            // 处理X切面
            let x_slice_begin_position = ((*line_x_start as f64).ceil()) as isize;
            let x_slice_end_position = ((*line_x_end as f64).ceil()) as isize;

            // 计算Y切面对应Y的层数
            let y_layer_index =
                ((*line_y as isize - px_position_y) / (y_layer.round() as isize)) as usize;

            let y_px = *line_y as isize - px_position_y; // Y 坐标

            for x_slice_layer in x_slice_begin_position..x_slice_end_position {
                let x_px = x_slice_layer - px_position_x; // X 坐标

                // 计算X切面对应X的层数
                let x_layer_index = (x_px / (x_layer.round() as isize)) as usize;

                // 生成Y切面
                result.y_rt[y_layer_index][(z as isize * *x as isize + x_px) as usize] = 1;
                // 生成X切面
                result.x_rt[x_layer_index][(z as isize * *y as isize + y_px) as usize] = 1;

                // 记录mask的边界
                // Y
                if result.y_bounds[y_layer_index].minx > x_px {
                    result.y_bounds[y_layer_index].minx = x_px
                }
                if result.y_bounds[y_layer_index].maxx < x_px {
                    result.y_bounds[y_layer_index].maxx = x_px
                }
                if result.y_bounds[y_layer_index].miny > z as isize {
                    result.y_bounds[y_layer_index].miny = z as isize
                }
                if result.y_bounds[y_layer_index].maxy < z as isize {
                    result.y_bounds[y_layer_index].maxy = z as isize
                }
                // X
                if result.x_bounds[x_layer_index].minx > y_px {
                    result.x_bounds[x_layer_index].minx = y_px
                }
                if result.x_bounds[x_layer_index].maxx < y_px {
                    result.x_bounds[x_layer_index].maxx = y_px
                }
                if result.x_bounds[x_layer_index].miny > z as isize {
                    result.x_bounds[x_layer_index].miny = z as isize
                }
                if result.x_bounds[x_layer_index].maxy < z as isize {
                    result.x_bounds[x_layer_index].maxy = z as isize
                }
            }
        }
    }
    return result;
}

pub fn mask_to_rt(all_mask: RTMask, bounds: &Bounds) -> Vec<Vec<Contours>> {
    let Bounds {
        x,
        y,
        z,
        // px_position_patient, // 每层的原点像素坐标
        ..
    } = bounds;

    let RTMask {
        x_rt,
        y_rt,
        x_bounds,
        y_bounds,
    } = all_mask;

    let mut result_data = Vec::new();

    // X截面
    for index in 0..x_rt.len() {
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
        let contours = trace_contours(Mask {
            data: mask_item,
            width: *y as isize,
            height: *z as isize,
            minx,
            miny,
            maxx,
            maxy,
        });
        result_data.push(contours);
    }

    // Y截面
    for index in 0..y_rt.len() {
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
        let contours = trace_contours(Mask {
            data: mask_item,
            width: *x as isize,
            height: *z as isize,
            minx,
            miny,
            maxx,
            maxy,
        });
        result_data.push(contours);
    }
    result_data
}

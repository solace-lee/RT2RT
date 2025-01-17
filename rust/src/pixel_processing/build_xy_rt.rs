use serde::{Deserialize, Serialize};

use crate::init_data::calc_rt_bounds::Bounds;

use super::magic_wand::{trace_contours, Contours, Mask};

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
pub fn generate_mask(line: Vec<Vec<i32>>, bounds: &Bounds) -> RTMask {
    let Bounds {
        x,
        y,
        z,
        x_layer, // x轴 像素/层
        y_layer, // y轴 像素/ 层
        ..
    } = bounds;
    let x_layer_num = (*x as f64 / x_layer).ceil(); // 计算X切面的层数
    let y_layer_num = (*y as f64 / y_layer).ceil(); // 计算Y切面的层数

    // 初始化mask
    let mut result = RTMask {
        x_rt: vec![vec![0; (y * (z + 1)) as usize]; x_layer_num as usize],
        y_rt: vec![vec![0; (x * (z + 1)) as usize]; y_layer_num as usize],
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

    for z in 0..line.len() {
        let layer_coords = &line[z]; // z层数

        for line_index in 0..(layer_coords.len() / 3) as usize {
            let line_x_start = &layer_coords[line_index * 3];
            let line_x_end = &layer_coords[line_index * 3 + 1];
            let line_y = &layer_coords[line_index * 3 + 2];
            // 处理X切面
            let x_slice_begin_position = ((*line_x_start as f64).ceil()) as isize;
            let x_slice_end_position = ((*line_x_end as f64).ceil()) as isize;

            // 计算Y切面对应Y的层数
            let y_layer_index = (*line_y as f64 / y_layer).round() as usize;

            let y_px = *line_y as isize; // Y 坐标

            for x_slice_layer in x_slice_begin_position..x_slice_end_position {
                // X 坐标

                // 计算X切面对应X的层数
                let x_layer_index = (x_slice_layer as f64 / x_layer).round() as usize;

                // 生成Y切面
                if (*line_y as f64 - (y_layer * y_layer_index as f64)).abs() < 0.5 {
                    result.y_rt[y_layer_index]
                        [(z as isize * *x as isize + x_slice_layer) as usize] = 1;
                    // 记录mask的边界
                    // Y
                    if result.y_bounds[y_layer_index].minx > x_slice_layer {
                        result.y_bounds[y_layer_index].minx = x_slice_layer
                    }
                    if result.y_bounds[y_layer_index].maxx < x_slice_layer {
                        result.y_bounds[y_layer_index].maxx = x_slice_layer
                    }
                    if result.y_bounds[y_layer_index].miny > z as isize {
                        result.y_bounds[y_layer_index].miny = z as isize
                    }
                    if result.y_bounds[y_layer_index].maxy < z as isize {
                        result.y_bounds[y_layer_index].maxy = z as isize
                    }
                }
                // 生成X切面
                if (x_slice_layer as f64 - (x_layer * x_layer_index as f64)).abs() < 0.5 {
                    result.x_rt[x_layer_index][(z as isize * *y as isize + y_px) as usize] = 1;
                    // 记录mask的边界
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
    }
    return result;
}

pub fn mask_to_rt(all_mask: RTMask, bounds: &Bounds) -> RTContours {
    let Bounds {
        x,
        y,
        z,
        px_position_patient, // 每层的原点像素坐标
        ..
    } = bounds;

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

    let layer_num = px_position_patient.len() / 3;

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
            width: *y as isize,
            height: *z as isize,
            minx,
            miny,
            maxx,
            maxy: maxy + 1,
        });
        // if index == 64 {
        //     println!("hh, {:#?}, {}, {}", contours, minx, maxx);
        //     output::output(&contours, "./json/xxx.json");
        // }

        for index in 0..contours.len() {
            // 轮廓数
            let length = contours[index].points.len();
            for i in 0..(length / 2) {
                // 遍历每个轮廓
                let y = i * 2;
                let z = contours[index].points[y + 1];
                let dy =
                    px_position_patient[((layer_num + z as usize) % layer_num) * 3 + 1] as isize;
                contours[index].points[y] = contours[index].points[y] + dy;
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

        // // 处理单条直线的问题
        // let mut minx = minx;
        // let mut maxx = maxx;
        // let mut miny = miny;
        // let mut maxy = maxy;
        // if maxx == minx {
        //     if maxx == *y as isize {
        //         minx -= 1;
        //     } else {
        //         maxx += 1;
        //     }
        // }
        // if maxy == miny {
        //     if maxy == *x as isize {
        //         miny -= 1;
        //     } else {
        //         maxy += 1;
        //     }
        // }

        // 提取mask的轮廓
        let mut contours = trace_contours(Mask {
            data: mask_item,
            width: *x as isize,
            height: *z as isize,
            minx,
            miny,
            maxx,
            maxy: maxy + 1,
        });
        for index in 0..contours.len() {
            // 轮廓数
            let length = contours[index].points.len();
            for i in 0..(length / 2) {
                // 遍历每个轮廓
                let x = i * 2;
                let z = contours[index].points[x + 1];
                let dx = px_position_patient[(layer_num + z as usize) % layer_num * 3] as isize;
                contours[index].points[x] = contours[index].points[x] + dx;
            }
        }
        result_data.y[index] = contours;
    }
    result_data
}

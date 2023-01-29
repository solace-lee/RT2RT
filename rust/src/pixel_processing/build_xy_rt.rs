use serde::{Deserialize, Serialize};

use crate::init_data::calc_rt_bounds::Bounds;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct RTResult {
    pub x_rt: Vec<Vec<i32>>,
    pub y_rt: Vec<Vec<i32>>,
}

// 构建
pub fn generate_mask(line: Vec<Vec<i32>>, bounds: Bounds) -> RTResult {
    let Bounds {
        x,
        y,
        z,
        x_layer,             // x轴 像素/层
        y_layer,             // y轴 像素/ 层
        px_position_patient, // 每层的原点像素坐标
        ..
    } = bounds;
    let x_layer_num = (x as f64 / x_layer.round()).ceil(); // 计算X切面的层数
    let y_layer_num = (y as f64 / y_layer.round()).ceil(); // 计算Y切面的层数

    // 初始化mask
    let mut result = RTResult {
        x_rt: vec![vec![0; (y * z) as usize]; x_layer_num as usize],
        y_rt: vec![vec![0; (x * z) as usize]; y_layer_num as usize],
    };

    println!(
        "X轴层数{:?},像素{:?} Y轴层数{:?}, 像素{:?}",
        result.x_rt.len(),
        x_layer.round(),
        result.y_rt.len(),
        y_layer.round(),
    );

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

            for x_slice_layer in x_slice_begin_position..x_slice_end_position {
                // 计算X切面对应X的层数
                let x_layer_index =
                    ((x_slice_layer - px_position_x) / (x_layer.round() as isize)) as usize;

                // 生成Y切面
                result.y_rt[y_layer_index][(z as isize * x as isize + (x_slice_layer - px_position_x)) as usize] = 1;
                // 生成X切面
                result.x_rt[x_layer_index][(z as isize * y as isize + (*line_y as isize - px_position_y)) as usize] =
                    1;
            }
            // result.x_rt
        }
    }
    return result;
}

pub fn build_xy_rt() {}

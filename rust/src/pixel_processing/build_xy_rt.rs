use crate::init_data::calc_rt_bounds::Bounds;


pub struct RTResult {
    pub x_rt: Vec<Vec<i32>>,
    pub y_rt: Vec<Vec<i32>>,
}

pub fn build_xy_rt(line: Vec<Vec<i32>>, bounds: Bounds) {
    let Bounds {
        x,
        y,
        z,
        x_layer, // x轴 像素/层
        y_layer, // y轴 像素/ 层
        ..
    } = bounds;
    let x_layer_num = (x as f64 / x_layer.round()).ceil();
    let y_layer_num = (y as f64 / y_layer.round()).ceil();

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
        let layer_coords = &line[z];
        for line_index in 0..(layer_coords.len() / 3) as usize {
            let line_x_start = &layer_coords[line_index * 3];
            let line_x_end = &layer_coords[line_index * 3 + 1];
            let line_y = &layer_coords[line_index * 3 + 2];
            // 处理X切面
            let x_slice_begin_position = ((*line_x_start as f64 / x_layer.round()).ceil()) as isize;
            let x_slice_end_position = ((*line_x_end as f64 / x_layer.round()).ceil()) as isize;

            for x_slice_layer in x_slice_begin_position..x_slice_end_position {}
            // result.x_rt
        }
    }
}

pub fn generate_mask() {}

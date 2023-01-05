use crate::volume_tools::volume::volume::Bounds;

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
    } = bounds;
    let x_layer_num = (x as f64 / x_layer.round()).ceil();
    let y_layer_num = (y as f64 / y_layer.round()).ceil();

    let result = RTResult {
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
}

use crate::init_data::calc_rt_bounds::{PixelCoods, PxData};

/// 验证生成的轮廓是否连续
fn check_result(begin: usize, end: usize, coords: &Vec<PixelCoods>) -> bool {
    if begin == end {
        return true;
    }
    let mut pass = true;
    let mut i = begin - 1;
    let mut p_x = coords[i].x.abs();
    let mut p_y = coords[i].y.abs();
    loop {
        let x = coords[i].x.abs();
        let y = coords[i].y.abs();

        if (p_x - x).abs() <= 1 {
            p_x = x;
        } else {
            pass = false;
            break;
        }

        if (p_y - y).abs() <= 1 {
            p_y = y;
        } else {
            pass = false;
            break;
        }

        if i == (end - 1) {
            break;
        }
        i += 1;
    }
    return pass;
}

/// 像素插值（结果不包含second）
fn insert_coord(first: &PixelCoods, second: &PixelCoods, result: &mut Vec<PixelCoods>) {
    if result.len() > 1 {
        let last = result[result.len() - 1]; // 如果上一个点和这个点一样，则忽略
        if !(last.x == first.x && last.y == first.y) {
            result.push(*first);
        }
    } else {
        result.push(*first); // 放入第一个点
    }
    let begin = result.len();

    let first_x = first.x;
    let first_y = first.y;
    let second_x = second.x;
    let second_y = second.y;

    let sub_x = second_x - first_x; // 判断X方向上的差异
    let sub_y = second_y - first_y; // 判断Y方向上的差异

    // 取绝对值找到最大的差异
    let max_count;
    if sub_y.abs() > sub_x.abs() {
        max_count = "y"
    } else {
        max_count = "x"
    }

    // 按差异最大的方向开始线性补点
    if max_count == "y" {
        let y_step;
        let x_step;
        if sub_y.abs() == 0 {
            // 排除除数为0的情况
            y_step = 0;
            x_step = 0.0;
        } else {
            y_step = sub_y / sub_y.abs();
            x_step = sub_x as f32 / sub_y.abs() as f32; // 计算另一个方向上的步进分量
        }

        for i in 1..sub_y.abs() {
            // 按差异最大方向逐行补点
            let item = PixelCoods {
                x: (first_x as f32 + (i as f32 * x_step)).round() as i32,
                y: first_y + i * y_step,
            };

            let last = result[result.len() - 1]; // 如果上一个点和这个点一样，则忽略
            if !(last.x == item.x && last.y == item.y) {
                result.push(item)
            }
        }
    } else {
        let y_step;
        let x_step;
        if sub_x.abs() == 0 {
            x_step = 0;
            y_step = 0.0;
        } else {
            y_step = sub_y as f32 / sub_x.abs() as f32;
            x_step = sub_x / sub_x.abs();
        }

        for i in 1..sub_x.abs() {
            let item = PixelCoods {
                y: (first_y as f32 + i as f32 * y_step).round() as i32,
                x: first_x + i * x_step,
            };

            let last = result[result.len() - 1];
            if !(last.x == item.x && last.y == item.y) {
                result.push(item)
            }
        }
    }

    let end = result.len();
    let is_pass = check_result(begin, end, result);
    if is_pass == false {
        println!("两点间结果通过：{}", is_pass.to_string());
    }
}

/// 生成闭合曲线
pub fn closed_line(pixel_data: PxData) -> PxData {
    let mut line = PxData {
        data: Vec::new(),
        bounds: pixel_data.bounds,
    };
    let PxData { data, .. } = pixel_data;

    for layer in data {
        let mut layer_result = Vec::new();
        if layer.len() != 0 {
            // 如果当前层存在轮廓数据

            for coords in layer {
                let mut coords_result = Vec::new();
                if coords.len() != 0 {
                    // 如果当前轮廓有坐标数据

                    let mut i = 0;
                    loop {
                        let mut next_index = i + 1;
                        if next_index == coords.len() {
                            next_index = 0
                        }
                        let first = &coords[i];
                        let second = &coords[next_index];

                        insert_coord(first, second, &mut coords_result);

                        if next_index == 0 {
                            break;
                        } else {
                            i += 1
                        }
                    }
                }
                layer_result.push(coords_result)
            }
        }
        line.data.push(layer_result);
    }

    return line;
}

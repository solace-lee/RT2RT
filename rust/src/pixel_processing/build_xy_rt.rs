use serde::{Deserialize, Serialize};

use crate::init_data::init_json::ImageInfo;

use super::magic_wand::{Contours, Mask, Point, trace_contours};

use super::smooth::{Options, Vec4Point, contour_smooth_by_level, smooth_by_radius};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RTContours {
    pub x: Vec<Vec<Vec<Point>>>,
    pub y: Vec<Vec<Vec<Point>>>
}

// 基于线数据构建层mask
pub fn generate_mask(mask_volume: &Vec<i8>, bounds: &ImageInfo) -> RTMask {
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
        x_rt: vec![vec![0; (row * lay_num) as usize]; x_layer_num as usize],
        y_rt: vec![vec![0; (column * lay_num) as usize]; y_layer_num as usize],
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
                let is_x_true = (current_x_layer * x_layer).floor() as u32 == x_num;
                if is_x_true {
                    let lay_index = current_x_layer as usize;

                    if result.x_bounds[lay_index].minx > y_num as isize {
                        result.x_bounds[lay_index].minx = y_num as isize
                    }
                    if result.x_bounds[lay_index].maxx < y_num as isize {
                        result.x_bounds[lay_index].maxx = y_num as isize
                    }
                    if result.x_bounds[lay_index].miny > z_num as isize {
                        result.x_bounds[lay_index].miny = z_num as isize
                    }
                    if result.x_bounds[lay_index].maxy < z_num as isize {
                        result.x_bounds[lay_index].maxy = z_num as isize
                    }
                    result.x_rt[lay_index][(z_num * *row + y_num) as usize] = 1;
                }

                // 生成y切面
                let is_y_true = (current_y_layer * y_layer).floor() as u32 == y_num;
                if is_y_true {
                    let lay_index = current_y_layer as usize;

                    if result.y_bounds[lay_index].minx > x_num as isize {
                        result.y_bounds[lay_index].minx = x_num as isize;
                    }
                    if result.y_bounds[lay_index].maxx < x_num as isize {
                        result.y_bounds[lay_index].maxx = x_num as isize;
                    }
                    if result.y_bounds[lay_index].miny > z_num as isize {
                        result.y_bounds[lay_index].miny = z_num as isize;
                    }
                    if result.y_bounds[lay_index].maxy < z_num as isize {
                        result.y_bounds[lay_index].maxy = z_num as isize;
                    }
                    result.y_rt[lay_index][(z_num * *column + x_num) as usize] = 1;
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
        y: Vec::new()
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
        let contours = trace_contours(Mask {
            data: mask_item,
            width: *row as isize,
            height: *lay_num as isize,
            minx,
            miny,
            maxx,
            maxy: maxy + 1,
        });
        // print!("x_layer: {} contours: {:?}", index, contours);

        result_data.x[index] = smooth_rt(&contours, *pixel_spacing_normalized);
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
        let contours = trace_contours(Mask {
            data: mask_item,
            width: *column as isize,
            height: *lay_num as isize,
            minx,
            miny,
            maxx,
            maxy: maxy + 1,
        });

        result_data.y[index] = smooth_rt(&contours, *pixel_spacing_normalized);
    }
    result_data
}

fn smooth_rt(contours: &Vec<Contours>, pixel_spacing_normalized: f64) -> Vec<Vec<Point>> {
    let mut edge_coords: Vec<Vec<Point>> = Vec::new();
    let distance = pixel_spacing_normalized * 0.4;
    for item in contours {
        let points = smooth_by_radius(&item.points, &Options { radius: 2.0 });
        let mut coords: Vec<Vec4Point> = Vec::new();
        let len = points.len();
        for i in 0..len {
            if i == 0 {
                let p = &points[i];
                let y = p.y * pixel_spacing_normalized;
                coords.push(Vec4Point {
                    x: p.x + 1.0,
                    y,
                    z: 0.0,
                    old_y: y,
                });
            }
            let next_raw_p = &points[(i + 1 + len) % len];
            let next_y = next_raw_p.y * pixel_spacing_normalized;
            let mut next_p: Vec4Point = Vec4Point {
                x: next_raw_p.x + 1.0,
                y: next_y,
                z: 0.0,
                old_y: next_y,
            };

            let current_point = coords[i];
            // 处理单层问题
            if current_point.old_y == next_p.y {
                if current_point.x > next_p.x {
                    if current_point.old_y == current_point.y {
                        coords[i].y += distance;
                    }
                    next_p.y += distance;
                } else {
                    if current_point.old_y == current_point.y {
                        coords[i].y -= distance;
                    }
                    next_p.y -= distance;
                }
            }

            coords.push(next_p);
        }
        let smooth_coord: Vec<Point> = contour_smooth_by_level(&coords, 2.0);
        // let test = coords.iter().map(|p| Point { x: p.x, y: p.y }).collect();
        edge_coords.push(smooth_coord);
    }
    edge_coords
}

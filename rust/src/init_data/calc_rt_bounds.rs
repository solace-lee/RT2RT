use crate::init_data::init_json::ImageInfo;

/// 轮廓像素坐标点结构定义
#[derive(Debug, Clone, Copy)]
pub struct PixelCoods {
    pub x: i32,
    pub y: i32,
}

/// 轮廓像素坐标集合及边界
pub struct PxData {
    pub data: Vec<Vec<Vec<PixelCoods>>>,
    pub bounds: BoundsLimit,
}

///轮廓像素边界定义
#[derive(Debug)]
pub struct BoundsLimit {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

///体边界定义
#[derive(Debug)]
pub struct Bounds {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

///寻找XYZ的最小像素间距
pub fn find_pixel_spacing(arr: Vec<f64>) -> f64 {
    let min = arr.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
    *min
}

///计算volume的边界
pub fn get_volume_bounds(imagainfo: &ImageInfo, minpx: &f64) -> Bounds {
    let ImageInfo {
        column,
        row,
        lay_num,
        row_pixel_spacing,
        column_pixel_spacing,
        thickness,
        ..
    } = imagainfo;
    Bounds {
        x: (row_pixel_spacing / minpx * *row as f64).ceil() as u32,
        y: (column_pixel_spacing / minpx * *column as f64).ceil() as u32,
        z: (thickness * *lay_num as f64 / minpx).ceil() as u32,
    }
}

///物理坐标转像素坐标并计算轮廓边界
pub fn get_rt_pxdata_and_bounds(imagainfo: &ImageInfo) -> PxData {
    let ImageInfo {
        data,
        row_pixel_spacing,
        column_pixel_spacing,
        ..
    } = imagainfo;

    // let result = data.iter().iter().iter().for_each(|v| )

    let mut result = Vec::new();
    let mut bounds = BoundsLimit {
        min_x: 0,
        max_x: 0,
        min_y: 0,
        max_y: 0,
    };

    for i in data {
        let mut o = Vec::new();
        if i.len() != 0 {
            for j in i {
                let mut p = Vec::new();
                if j.len() != 0 {
                    for k in j {
                        let x = (k.x / row_pixel_spacing).ceil() as i32;
                        let y = (k.y / column_pixel_spacing).ceil() as i32;

                        if bounds.max_x < x {
                            bounds.max_x = x
                        } else if bounds.min_x > x {
                            bounds.min_x = x
                        }
                        if bounds.max_y < y {
                            bounds.max_y = y
                        } else if bounds.min_y > y {
                            bounds.min_y = y
                        }

                        p.push(PixelCoods { x, y });
                    }
                };
                o.push(p);
            }
        };
        result.push(o);
    }

    PxData {
        data: result,
        bounds,
    }
}

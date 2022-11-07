pub mod bounds {
    use crate::init_data::init_json::init_json::ImageInfo;

    pub fn find_pixel_spacing(arr: Vec<f64>) -> f64 {
        let min = arr.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
        *min
    }

    #[derive(Debug)]
    pub struct Bounds {
        pub x: u32,
        pub y: u32,
        pub z: u32,
    }

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

    #[derive(Debug)]
    pub struct PixelCoods {
        x: i32,
        y: i32,
    }

    pub struct PxData {
        pub data: Vec<Vec<Vec<PixelCoods>>>,
        pub bounds: PixelCoods,
    }

    pub fn get_rt_pxdata_and_bounds(imagainfo: &ImageInfo) -> PxData {
        let ImageInfo {
            data,
            row_pixel_spacing,
            column_pixel_spacing,
            ..
        } = imagainfo;

        // let result = data.iter().iter().iter().for_each(|v| )

        let mut result = Vec::new();
        let mut bounds = PixelCoods { x: 0, y: 0 };

        for i in data {
            let mut o = Vec::new();
            if i.len() != 0 {
                for j in i {
                    let mut p = Vec::new();
                    if j.len() != 0 {
                        for k in j {
                            let x = (k.x / row_pixel_spacing).ceil() as i32;
                            let y = (k.y / column_pixel_spacing).ceil() as i32;

                            if bounds.x < x.abs() {
                                bounds.x = x.abs()
                            }
                            if bounds.y < y.abs() {
                                bounds.y = y.abs()
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
}

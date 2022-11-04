extern crate serde_json;

pub mod init_json {
    use std::fs::File;

    #[derive(Debug)]
    pub enu imageInfo {
        pub column: u64,
        row: u64,
        lay_num: u64,
        row_pixel_spacing: f64,
        column_pixel_spacing: f64,
        thickness: f64,
        data: (),
    }

    impl imageInfo {
        pub fn new(path: &str) -> imageInfo {
            let f = File::open(path).unwrap();
            let v: serde_json::Value = serde_json::from_reader(f).unwrap();

            imageInfo {
                column: v["column"].as_u64().unwrap(),
                row: v["row"].as_u64().unwrap(),
                lay_num: v["layNum"].as_u64().unwrap(),
                row_pixel_spacing: v["rowPixelSpacing"].as_f64().unwrap(),
                column_pixel_spacing: v["columnPixelSpacing"].as_f64().unwrap(),
                thickness: v["thickness"].as_f64().unwrap(),
                data: (),
            }
        }
    }
}

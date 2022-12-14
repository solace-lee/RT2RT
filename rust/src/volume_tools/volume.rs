pub mod volume {
    use std::cell::RefCell;

    #[derive(Clone, Debug)]
    pub struct Bounds {
        pub x: u32,
        pub y: u32,
        pub z: u32,
        // pub z_pixel_spacing: f32,
    }

    pub struct Position {
        begin: u32,
        end: u32,
    }

    #[derive(Clone, Debug)]
    pub struct Volume {
        // pub data: Vec<Vec<u8>>, // 512*512*388的初始化性能比Vec<u128>慢10倍左右
        pub data: RefCell<Vec<u128>>, // 每2位标识一个轮廓，高位表方向，低位表是否有轮廓，01表示逆时针，11表示顺时针
        pub bounds: Bounds,
        pub name_map: Vec<String>,
    }

    impl Volume {
        pub fn new(bounds: Bounds) -> Volume {
            let volume_str = Volume {
                data: RefCell::new(vec![0; (bounds.x * bounds.y * bounds.z) as usize]),
                bounds,
                name_map: Vec::new(),
            };
            return volume_str;
        }

        // 构建实心volume
        pub fn build_volume(&self, rs_index: u8) {
            for layer in 0..self.bounds.z {
                let Position { begin, .. } = self.get_layer_position(layer);
                for row in 0..self.bounds.y {
                    let row_begin = begin + row * self.bounds.x;
                    // 逐行扫描
                    for column in 0..self.bounds.x {
                        let data_index = row_begin + column;
                        let item_value = self.data.borrow()[data_index as usize];
                        // 扫描的可能是极值点，也可能是中间点
                        
                    }
                }
            }
        }

        /// 修改坐标值
        pub fn set_pixel(&self, x: u32, y: u32, z: u32, value: u128) -> bool {
            if x > self.bounds.x || y > self.bounds.y || z > self.bounds.z {
                return false;
            }
            self.data.borrow_mut()
                [(z * self.bounds.x * self.bounds.y + y * self.bounds.x + x) as usize] = value;
            return true;
        }

        /// 获取层数据
        pub fn get_layer_data(&self, laynum: u32) -> Vec<u128> {
            let mut data: Vec<u128> = Vec::new();
            let Position { begin, end } = self.get_layer_position(laynum);

            let mut i = begin;
            loop {
                data.push(self.data.borrow()[i as usize]);
                i += 1;
                if i == end {
                    break;
                }
            }

            return data;
        }

        /// 获取层的起始和结束的下标
        pub fn get_layer_position(&self, laynum: u32) -> Position {
            let layerlimit = self.bounds.x * self.bounds.y;
            let begin = laynum * layerlimit;
            return Position {
                begin,
                end: begin + layerlimit,
            };
        }
    }
}

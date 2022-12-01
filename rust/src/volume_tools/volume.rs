pub mod volume {
    #[derive(Clone, Debug)]
    pub struct Bounds {
        pub x: u32,
        pub y: u32,
        pub z: u32,
    }

    pub struct Position {
        begin: u32,
        end: u32,
    }

    #[derive(Clone, Debug)]
    pub struct Volume {
        // pub data: Vec<Vec<u8>>, // 512*512*388的初始化性能比Vec<u128>慢10倍左右
        pub data: Vec<u128>,
        pub bounds: Bounds,
        pub name_map: Vec<String>,
    }

    impl Volume {
        pub fn new(bounds: Bounds) -> Volume {
            let volume_str = Volume {
                data: vec![0; (bounds.x * bounds.y * bounds.z) as usize],
                bounds,
                name_map: Vec::new(),
            };
            return volume_str;
        }

        /// 修改坐标值
        pub fn set_pixel(&mut self, x: u32, y: u32, z: u32, value: u128) -> bool {
            if x > self.bounds.x || y > self.bounds.y || z > self.bounds.z {
                return false;
            }
            self.data[(z * self.bounds.x * self.bounds.y + y * self.bounds.y + x) as usize] = value;
            return true;
        }

        /// 获取层数据
        pub fn get_layer_data(&self, laynum: u32) -> Vec<u128> {
            let mut data: Vec<u128> = Vec::new();
            let Position { begin, end } = self.get_layer_position(laynum);

            let mut i = begin;
            loop {
                data.push(self.data[i as usize]);
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

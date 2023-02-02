pub mod volume {
    use std::cell::RefCell;

    use crate::init_data::calc_rt_bounds::Bounds;

    

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

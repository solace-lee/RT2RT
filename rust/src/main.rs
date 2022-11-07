extern crate serde_json;

use rt2rt::init_data::init_json::init_json;

fn main() {
    let result = init_json::ImageInfo::new("./src/RT_fmt.json").expect("出现错误");
    println!("result{:?}", result);
}

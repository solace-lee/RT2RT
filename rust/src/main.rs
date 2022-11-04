extern crate serde_json;

use rt2rt::init_data::init_json::init_json;

fn main() {
    let result = init_json::imageInfo::new("./src/RT.json");
    println!("result{:?}", result.column);
}

extern crate serde_json;

use std::fs::File;

fn main() {
    let f = File::open("RT.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    println!("{:?}", v["column"].as_str().unwrap());
    println!("{:?}", v["row"].as_str().unwrap());
    println!("Hello, world!");
}

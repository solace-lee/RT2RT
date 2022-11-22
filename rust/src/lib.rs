use wasm_bindgen::prelude::*;
pub mod init_data;
pub mod pixel_processing;
pub mod output_json;

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

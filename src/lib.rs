use wasm_bindgen::prelude::*;
mod core;

#[wasm_bindgen]
pub fn test_func(arg1: u32) -> u32 {
    arg1 + 2
}

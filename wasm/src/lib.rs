use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

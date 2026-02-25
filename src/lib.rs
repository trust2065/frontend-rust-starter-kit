use wasm_bindgen::prelude::*;

// 讓 JS 可以呼叫這個函式
#[wasm_bindgen]
pub fn add_easy(a: i32, b: i32) -> i32 {
    a + b
}
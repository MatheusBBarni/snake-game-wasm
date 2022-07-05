use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) {
    println!("{}", name)
}

#[wasm_bindgen]
pub fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

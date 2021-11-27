use wasm_bindgen::prelude::*;
// wasm_bindgen: JavaScript と Rust を bridge する library
// prelude: 関連するクラスを良い感じにインポートする宣言

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
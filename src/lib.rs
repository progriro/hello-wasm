use wasm_bindgen::prelude::*;
// wasm_bindgen: JavaScript と Rust を bridge する library
// prelude: 関連するクラスを良い感じにインポートする宣言

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    pub fn confirm(s: &str) -> bool;
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}

#[wasm_bindgen]
pub fn judge(n1: i32, n2: i32) {
    if confirm(&format!("{} + {} = {} is true?", n1, n2, add(n1, n2))) {
        alert("Yeah!");
    } else {
        alert("Ooops!");
    }
}
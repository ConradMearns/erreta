use wasm_bindgen::prelude::*;

// pub fn run() {
//     bare_bones();
//     using_a_macro();
//     using_web_sys();
// }

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
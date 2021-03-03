use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = term)]
    fn write(s: &str);
}

#[wasm_bindgen]
pub fn send_key(code: &str, ctrl: bool, shift: bool, alt: bool, meta: bool) {
}

pub fn xterm_write(text: &str) {
    write(text);
}

use core::Key;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = term)]
    fn write(s: &str);
    #[wasm_bindgen(js_namespace = term)]
    fn get_key() -> Option<String>;

}

pub fn xterm_write(text: &str) {
    write(text);
}

pub fn xterm_read() -> Key {
    match get_key() {
        Some(s) => Key::Char(s.chars().next().unwrap()),
        None => Key::Unknown,
    }
}

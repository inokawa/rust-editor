use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = term)]
    fn write(s: &str);
    #[wasm_bindgen(js_namespace = term)]
    fn get_key() -> Option<String>;
    #[wasm_bindgen(js_namespace = term)]
    fn get_col_size() -> usize;
    #[wasm_bindgen(js_namespace = term)]
    fn get_row_size() -> usize;
}

pub fn xterm_write(text: &str) {
    write(text);
}

pub fn xterm_read() -> Option<String> {
    get_key()
}

pub fn xterm_get_window_size() -> (usize, usize) {
    let col = get_col_size();
    let row = get_row_size();
    (row, col)
}

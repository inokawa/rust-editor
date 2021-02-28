pub mod filer;
pub mod input;
pub mod output;

use core::{Editor, Filer, Input, Output};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console::log_1(&JsValue::from_str("Hello world!"));

    let input = input::WebInput::new().unwrap();
    let mut editor = Editor::new(input, output::WebOutput::new(), filer::WebFile::new()).unwrap();
    editor.run();

    Ok(())
}

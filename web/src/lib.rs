pub mod filer;
pub mod input;
pub mod output;
pub mod xterm;

use core::{Editor, Filer, Input, Output};
use js_sys::{Error, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(
    inline_js = "export function timeout() { return new Promise((resolve)=> setTimeout(resolve)); }"
)]
extern "C" {
    fn timeout() -> Promise;
}

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    let input = input::WebInput::new().unwrap();
    let mut editor = Editor::new(input, output::WebOutput::new(), filer::WebFile::new()).unwrap();

    loop {
        JsFuture::from(timeout()).await?;
        let quit = editor
            .run()
            .map_err(|err| JsValue::from(Error::new(&format!("{:?}", err))))?;
        if quit {
            break;
        }
    }
    Ok(())
}

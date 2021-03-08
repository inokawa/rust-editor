pub mod filer;
pub mod input;
pub mod output;
pub mod xterm;

use core::{Editor, Filer, Input, Output};
use js_sys::global;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::WorkerGlobalScope;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let input = input::WebInput::new().unwrap();
    let mut editor = Editor::new(input, output::WebOutput::new(), filer::WebFile::new()).unwrap();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        editor.run();
        set_micro_task(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    set_micro_task(g.borrow().as_ref().unwrap());

    Ok(())
}

fn set_micro_task(f: &Closure<dyn FnMut()>) {
    let global = global().unchecked_into::<WorkerGlobalScope>();
    global.set_timeout_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), 0);
}

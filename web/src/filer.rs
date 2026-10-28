use core::{Error, Filer};
use std::io;
use wasm_bindgen::prelude::*;

// See `fs` in `web/js/worker.ts`.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = fs)]
    fn read(filename: &str) -> Option<String>;
    #[wasm_bindgen(js_namespace = fs)]
    fn write(filename: &str, content: &str);
}

pub struct WebFile {}

impl Filer for WebFile {
    fn new() -> Self {
        WebFile {}
    }

    fn load(&self, filename: &String) -> Result<String, Error> {
        read(filename).ok_or_else(|| io::Error::from(io::ErrorKind::NotFound).into())
    }

    fn save(&self, filename: &String, contents: Vec<String>) -> Result<(), Error> {
        let mut buf = String::new();
        for row in &contents {
            buf.push_str(row);
            buf.push('\n');
        }
        write(filename, &buf);
        Ok(())
    }
}

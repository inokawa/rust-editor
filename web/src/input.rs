use core::{Arrow, Command, Error, Input, Key, Page};
use js_sys::Reflect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Returns `InputEvent` in `web/js/protocol.ts`, or undefined if the queue is empty.
    #[wasm_bindgen(js_namespace = input, js_name = next)]
    fn next_event() -> JsValue;
}

fn get_string(event: &JsValue, key: &str) -> Option<String> {
    Reflect::get(event, &JsValue::from_str(key))
        .ok()
        .and_then(|v| v.as_string())
}

pub struct WebInput {}

impl Input for WebInput {
    fn new() -> Result<Self, Error> {
        Ok(WebInput {})
    }

    fn wait_for_key(&self) -> Key {
        let event = next_event();
        match (get_string(&event, "kind"), get_string(&event, "value")) {
            (Some(kind), Some(value)) => decode(&kind, value),
            _ => Key::Unknown,
        }
    }
}

fn decode(kind: &str, value: String) -> Key {
    match kind {
        "key" => match value.as_str() {
            "Escape" => Key::Escape,
            "Backspace" => Key::Backspace,
            "Delete" => Key::Del,
            "Enter" => Key::Enter,
            "Home" => Key::Home,
            "End" => Key::End,
            "PageUp" => Key::Page(Page::Up),
            "PageDown" => Key::Page(Page::Down),
            "ArrowUp" => Key::Arrow(Arrow::Up),
            "ArrowDown" => Key::Arrow(Arrow::Down),
            "ArrowLeft" => Key::Arrow(Arrow::Left),
            "ArrowRight" => Key::Arrow(Arrow::Right),
            "Tab" => Key::Char('\t'),
            _ => Key::Unknown,
        },
        "char" => match value.chars().next() {
            Some(c) if c.is_ascii() => Key::Char(c),
            Some(c) => Key::CharUtf8(c),
            None => Key::Unknown,
        },
        "command" => match value.as_str() {
            "find" => Key::Command(Command::Find),
            "undo" => Key::Command(Command::Undo),
            "redo" => Key::Command(Command::Redo),
            "save" => Key::Command(Command::Save),
            "exit" => Key::Command(Command::Exit),
            _ => Key::Unknown,
        },
        "open" => Key::Command(Command::Open(value)),
        _ => Key::Unknown,
    }
}

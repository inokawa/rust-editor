use super::xterm;
use core::{Arrow, Error, Input, Key, Page};

pub struct WebInput {}

impl Input for WebInput {
    fn new() -> Result<Self, Error> {
        Ok(WebInput {})
    }

    fn wait_for_key(&self) -> Key {
        match xterm::xterm_read() {
            Some(s @ _) => match s.as_str() {
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
                _ => Key::Char(s.chars().next().unwrap()),
            },
            None => Key::Unknown,
        }
    }
}

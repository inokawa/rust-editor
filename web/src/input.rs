use super::xterm;
use core::{Arrow, Command, Error, Input, Key, Page};

pub struct WebInput {}

impl Input for WebInput {
    fn new() -> Result<Self, Error> {
        Ok(WebInput {})
    }

    fn wait_for_key(&self) -> Key {
        let key = xterm::xterm_read();
        match key.0 {
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
                "Tab" => Key::Char('\t'),
                c @ _ => {
                    if key.1 == true {
                        match c {
                            "f" | "F" => return Key::Command(Command::Find),
                            "q" | "Q" => return Key::Command(Command::Exit),
                            "s" | "S" => return Key::Command(Command::Save),
                            "z" | "Z" => return Key::Command(Command::Undo),
                            "y" | "Y" => return Key::Command(Command::Redo),
                            "h" | "H" => return Key::Backspace,
                            "l" | "L" => return Key::Escape,
                            _ => {}
                        }
                    }
                    return Key::Char(s.chars().next().unwrap());
                }
            },
            None => Key::Unknown,
        }
    }
}

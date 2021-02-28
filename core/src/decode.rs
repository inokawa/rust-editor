use super::{Arrow, Command, Key, Page};
use std::str;

const fn ctrl(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}

const FIND: u8 = ctrl('f');
const EXIT: u8 = ctrl('q');
const SAVE: u8 = ctrl('s');
const UNDO: u8 = ctrl('z');
const REDO: u8 = ctrl('y');
const DELETE: u8 = ctrl('h');
const REFRESH_SCREEN: u8 = ctrl('l');

pub trait Decode {
    fn read(&self) -> Option<u8>;

    fn decode(&self) -> Key {
        let b: u8;
        loop {
            if let Some(res) = self.read() {
                b = res;
                break;
            }
        }
        match b {
            // ASCII 0x00~0x7f
            ctrl @ 0x00..=0x1f => match ctrl {
                0x1b => self.decode_escape_sequence(),
                b'\t' => Key::Char(ctrl as char),
                b'\r' | b'\n' => Key::Enter,
                DELETE => Key::Backspace,
                REFRESH_SCREEN => Key::Escape,
                FIND => Key::Command(Command::Find),
                UNDO => Key::Command(Command::Undo),
                REDO => Key::Command(Command::Redo),
                SAVE => Key::Command(Command::Save),
                EXIT => Key::Command(Command::Exit),
                _ => Key::Unknown,
            },
            0x20 => Key::Char(b as char),
            0x21..=0x7e => Key::Char(b as char),
            0x7f => Key::Backspace,
            // UTF-8 0x80~0xff
            0x80..=0xff => self.decode_utf8(b),
        }
    }

    fn decode_escape_sequence(&self) -> Key {
        // TODO ignore unhandled escape sequences
        match self.read() {
            Some(b'[') => {
                match self.read() {
                    Some(b) => match b {
                        b'A' => return Key::Arrow(Arrow::Up),
                        b'B' => return Key::Arrow(Arrow::Down),
                        b'C' => return Key::Arrow(Arrow::Right),
                        b'D' => return Key::Arrow(Arrow::Left),
                        b'H' => return Key::Home,
                        b'F' => return Key::End,
                        n @ b'0'..=b'9' => match self.read() {
                            Some(b'~') => match n {
                                b'1' | b'7' => return Key::Home,
                                b'4' | b'8' => return Key::End,
                                b'3' => return Key::Del,
                                b'5' => return Key::Page(Page::Up),
                                b'6' => return Key::Page(Page::Down),
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    },
                    None => {}
                }
                return Key::Unknown;
            }
            Some(b'O') => match self.read() {
                Some(b'H') => return Key::Home,
                Some(b'F') => return Key::End,
                _ => {}
            },
            _ => {}
        }
        Key::Escape
    }

    fn decode_utf8(&self, b: u8) -> Key {
        let mut buf: Vec<u8> = vec![b];

        while buf.len() < 4 {
            if let Some(b) = self.read() {
                buf.push(b);
            }
            if let Ok(s) = str::from_utf8(&buf) {
                if let Some(c) = s.chars().next() {
                    return Key::CharUtf8(c);
                }
                return Key::Unknown;
            }
        }
        Key::Unknown
    }
}

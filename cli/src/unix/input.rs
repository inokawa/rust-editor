use core::{ansi_escape::*, Arrow, Command, Error, Input, Key, Page};
use libc::{
    tcgetattr, tcsetattr, termios, BRKINT, CS8, ECHO, ICANON, ICRNL, IEXTEN, INPCK, ISIG, ISTRIP,
    IXON, OPOST, STDIN_FILENO, TCSAFLUSH, VMIN, VTIME,
};
use std::{
    io::{self, Read},
    str,
};

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

#[cfg(target_os = "linux")]
fn init_term() -> termios {
    termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0u8; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    }
}
#[cfg(target_os = "macos")]
fn init_term() -> termios {
    termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_cc: [0u8; 20],
        c_ispeed: 0,
        c_ospeed: 0,
    }
}

#[cfg(unix)]
pub struct StdinRaw {
    orig: termios,
}

impl Input for StdinRaw {
    fn new() -> Result<Self, Error> {
        let mut term = init_term();
        unsafe { tcgetattr(STDIN_FILENO, &mut term) };

        let orig = term;

        // Set terminal raw mode. Disable echo back, canonical mode, signals (SIGINT, SIGTSTP) and Ctrl+V.
        term.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
        // Disable control flow mode (Ctrl+Q/Ctrl+S) and CR-to-NL translation
        term.c_iflag &= !(IXON | ICRNL | BRKINT | INPCK | ISTRIP);
        // Disable output processing such as \n to \r\n translation
        term.c_oflag &= !OPOST;
        // Ensure character size is 8bits
        term.c_cflag |= CS8;
        // Do not wait for next byte with blocking since reading 0 byte is permitted
        term.c_cc[VMIN] = 0;
        // Set read timeout to 1/10 second it enables 100ms timeout on read()
        term.c_cc[VTIME] = 1;
        // Apply terminal configurations
        unsafe { tcsetattr(STDIN_FILENO, TCSAFLUSH, &term) };

        print!("{}", SMCUP);

        Ok(StdinRaw { orig })
    }

    fn wait_for_key(&self) -> Key {
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
}

impl StdinRaw {
    fn read(&self) -> Option<u8> {
        if let Some(b) = io::stdin().bytes().next() {
            b.map(|b| Some(b)).unwrap_or(None)
        } else {
            None
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

impl Drop for StdinRaw {
    fn drop(&mut self) {
        print!("{}", RMCUP);

        unsafe { tcsetattr(STDIN_FILENO, TCSAFLUSH, &self.orig) };
    }
}

use super::{error::Error, input_unix::StdinRaw, output_unix::get_window_size};
use std::io::{self, Write};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const CLEAR_SCREEN: &str = "\x1b[2J";
const CLEAR_LINE_RIGHT_OF_CURSOR: &str = "\x1b[K";
const MOVE_CURSOR_TO_START: &str = "\x1b[H";
const HIDE_CURSOR: &str = "\x1b[?25l";
const SHOW_CURSOR: &str = "\x1b[?25h";

const fn ctrl(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}

const EXIT: u8 = ctrl('q');

enum Key {
    Escape,
    Exit,
    Del,
    Home,
    End,
    Page(Page),
    Arrow(Arrow),
    Char(u8),
}

enum Page {
    Up,
    Down,
}

enum Arrow {
    Up,
    Down,
    Left,
    Right,
}

struct Position {
    x: usize,
    y: usize,
}

struct Row {
    chars: String,
}

pub struct Editor {
    input: StdinRaw,
    screen_rows: usize,
    screen_cols: usize,
    cursor: Position,
    num_rows: usize,
    row: Row,
}

impl Editor {
    pub fn new(stdin: StdinRaw) -> Result<Self, Error> {
        if let Some((screen_rows, screen_cols)) = get_window_size() {
            Ok(Editor {
                input: stdin,
                screen_rows,
                screen_cols,
                cursor: Position { x: 0, y: 0 },
                num_rows: 0,
                row: Row {
                    chars: String::from(""),
                },
            })
        } else {
            Err(Error::Init)
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.draw_init();
        loop {
            self.refresh_screen()?;
            let quit = self.process_key_press()?;
            if quit == true {
                let mut buf = String::new();
                buf.push_str(CLEAR_SCREEN);
                buf.push_str(MOVE_CURSOR_TO_START);
                print!("{}", buf);
                break;
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        let mut buf = String::new();
        buf.push_str(HIDE_CURSOR);
        buf.push_str(MOVE_CURSOR_TO_START);

        self.draw_rows(&mut buf);

        buf.push_str(MOVE_CURSOR_TO_START);
        buf.push_str(&format!(
            "\x1b[{};{}H",
            self.cursor.y + 1,
            self.cursor.x + 1
        ));
        buf.push_str(SHOW_CURSOR);

        print!("{}", buf);
        io::stdout().flush()?;
        Ok(())
    }

    fn process_key_press(&mut self) -> Result<bool, Error> {
        if let Ok(key) = self.decode_sequence() {
            match key {
                Key::Del => {}
                Key::Home => self.cursor.x = 0,
                Key::End => self.cursor.x = self.screen_cols - 1,
                Key::Page(k) => {
                    let mut times = self.screen_rows;
                    while times > 0 {
                        self.move_cursor(match k {
                            Page::Up => Arrow::Up,
                            Page::Down => Arrow::Down,
                        });
                        times -= 1;
                    }
                }
                Key::Arrow(k) => {
                    self.move_cursor(k);
                }
                Key::Exit => return Ok(true),
                _ => {}
            }
        }
        Ok(false)
    }

    fn move_cursor(&mut self, key: Arrow) {
        match key {
            Arrow::Up if self.cursor.y > 0 => self.cursor.y -= 1,
            Arrow::Down if self.cursor.y < self.screen_rows - 1 => self.cursor.y += 1,
            Arrow::Left if self.cursor.x > 0 => self.cursor.x -= 1,
            Arrow::Right if self.cursor.x < self.screen_cols - 1 => self.cursor.x += 1,
            _ => {}
        }
    }

    fn decode_sequence(&mut self) -> Result<Key, Error> {
        let b: u8;
        loop {
            if let Some(res) = self.input.read() {
                b = res;
                break;
            }
        }
        match b {
            b'\x1b' => {
                match self.input.read() {
                    Some(b'[') => match self.input.read() {
                        Some(b'A') => return Ok(Key::Arrow(Arrow::Up)),
                        Some(b'B') => return Ok(Key::Arrow(Arrow::Down)),
                        Some(b'C') => return Ok(Key::Arrow(Arrow::Right)),
                        Some(b'D') => return Ok(Key::Arrow(Arrow::Left)),
                        Some(b'H') => return Ok(Key::Home),
                        Some(b'F') => return Ok(Key::End),
                        Some(b'3') => match self.input.read() {
                            Some(b'~') => return Ok(Key::Del),
                            _ => {}
                        },
                        Some(b'1') | Some(b'7') => match self.input.read() {
                            Some(b'~') => return Ok(Key::Home),
                            _ => {}
                        },
                        Some(b'4') | Some(b'8') => match self.input.read() {
                            Some(b'~') => return Ok(Key::End),
                            _ => {}
                        },
                        Some(b'5') => match self.input.read() {
                            Some(b'~') => return Ok(Key::Page(Page::Up)),
                            _ => {}
                        },
                        Some(b'6') => match self.input.read() {
                            Some(b'~') => return Ok(Key::Page(Page::Down)),
                            _ => {}
                        },
                        _ => {}
                    },
                    Some(b'O') => match self.input.read() {
                        Some(b'H') => return Ok(Key::Home),
                        Some(b'F') => return Ok(Key::End),
                        _ => {}
                    },
                    _ => {}
                }
                Ok(Key::Escape)
            }
            b'w' => Ok(Key::Arrow(Arrow::Up)),
            b's' => Ok(Key::Arrow(Arrow::Down)),
            b'a' => Ok(Key::Arrow(Arrow::Left)),
            b'd' => Ok(Key::Arrow(Arrow::Right)),
            EXIT => Ok(Key::Exit),
            _ => Ok(Key::Char(b)),
        }
    }

    fn draw_rows(&self, buf: &mut String) {
        let width = self.screen_cols;
        let height = self.screen_rows;
        for y in 0..height {
            if y >= self.num_rows {
                if y == height / 3 {
                    let message = format!("Kilo editor -- version {}", VERSION);
                    let padding = width.saturating_sub(message.len()) / 2;
                    let spaces = " ".repeat(padding.saturating_sub(1));
                    let mut message = format!("~{}{}", spaces, message);
                    message.truncate(width);
                    buf.push_str(&message);
                } else {
                    buf.push_str("~");
                }
            } else {
                let mut chars = self.row.chars.clone();
                chars.truncate(width);
                buf.push_str(&chars);
            }

            buf.push_str(CLEAR_LINE_RIGHT_OF_CURSOR);
            if y < height - 1 {
                buf.push_str("\r\n");
            }
        }
    }

    fn draw_init(&mut self) {
        self.row.chars = "Hello, world!\0".to_string();
        self.num_rows = 1;
    }
}

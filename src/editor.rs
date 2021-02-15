use super::{
    ansi_escape::*, document::Document, error::Error, input_unix::StdinRaw,
    output_unix::get_window_size,
};
use std::{
    cmp,
    io::{self, Write},
    time::{Duration, Instant},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const fn ctrl(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}

const ESCAPE: u8 = b'\x1b';
const EXIT: u8 = ctrl('q');
const SAVE: u8 = ctrl('s');
const DELETE_BIS: u8 = ctrl('h');
const REFRESH_SCREEN: u8 = ctrl('l');
const BACKSPACE: u8 = 127;

enum Key {
    Escape,
    Exit,
    Save,
    Backspace,
    Del,
    Enter,
    Home,
    End,
    Page(Page),
    Arrow(Arrow),
    Char(u8),
    None,
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

struct Screen {
    rows: usize,
    cols: usize,
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}

struct Message {
    text: String,
    time: Instant,
}

impl Message {
    fn new(text: String) -> Self {
        Message {
            text,
            time: Instant::now(),
        }
    }
}

pub struct Editor {
    input: StdinRaw,
    screen: Screen,
    cursor: Position,
    row_offset: usize,
    col_offset: usize,
    document: Document,
    message: Option<Message>,
}

impl Editor {
    pub fn new(stdin: StdinRaw, document: Document) -> Result<Self, Error> {
        if let Some((screen_rows, screen_cols)) = get_window_size() {
            Ok(Editor {
                input: stdin,
                screen: Screen {
                    rows: screen_rows - 2,
                    cols: screen_cols,
                },
                cursor: Position { x: 0, y: 0 },
                row_offset: 0,
                col_offset: 0,
                document,
                message: Some(Message::new(String::from("HELP: Ctrl-Q = quit"))),
            })
        } else {
            Err(Error::Init)
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
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

    fn refresh_screen(&mut self) -> Result<(), Error> {
        self.scroll();

        let mut buf = String::new();
        buf.push_str(HIDE_CURSOR);
        buf.push_str(MOVE_CURSOR_TO_START);

        self.draw_rows(&mut buf);
        self.draw_status_bar(&mut buf);
        self.draw_message_bar(&mut buf);

        buf.push_str(MOVE_CURSOR_TO_START);
        buf.push_str(&format!(
            "\x1b[{};{}H",
            (self.cursor.y - self.row_offset) + 1,
            (self.cursor.x - self.col_offset) + 1
        ));
        buf.push_str(SHOW_CURSOR);

        print!("{}", buf);
        io::stdout().flush()?;
        Ok(())
    }

    fn scroll(&mut self) {
        if self.cursor.y < self.row_offset {
            self.row_offset = self.cursor.y;
        }
        if self.cursor.y >= self.row_offset + self.screen.rows {
            self.row_offset = self.cursor.y - self.screen.rows + 1;
        }
        if self.cursor.x < self.col_offset {
            self.col_offset = self.cursor.x;
        }
        if self.cursor.x >= self.col_offset + self.screen.cols {
            self.col_offset = self.cursor.x - self.screen.cols + 1;
        }
    }

    fn process_key_press(&mut self) -> Result<bool, Error> {
        if let Ok(key) = self.decode_sequence() {
            match key {
                Key::Escape => {}
                Key::Enter => {}
                Key::Backspace => {}
                Key::Del => {}
                Key::Home => self.cursor.x = 0,
                Key::End => {
                    if let Some(row) = self.document.row(self.cursor.y) {
                        self.cursor.x = row.len();
                    }
                }
                Key::Page(k) => {
                    let direction = match k {
                        Page::Up => {
                            self.cursor.y = self.row_offset;
                            Arrow::Up
                        }
                        Page::Down => {
                            self.cursor.y = self.row_offset + self.screen.rows - 1;
                            if self.cursor.y > self.document.len() {
                                self.cursor.y = self.document.len();
                            }
                            Arrow::Down
                        }
                    };
                    let mut times = self.screen.rows;
                    while times > 0 {
                        self.move_cursor(&direction);
                        times -= 1;
                    }
                }
                Key::Arrow(k) => {
                    self.move_cursor(&k);
                }
                Key::Save => {}
                Key::Exit => return Ok(true),
                Key::Char(c) => self.document.insert(c as char, &self.cursor),
                _ => {}
            }
        }
        Ok(false)
    }

    fn move_cursor(&mut self, key: &Arrow) {
        match key {
            Arrow::Up if self.cursor.y > 0 => self.cursor.y -= 1,
            Arrow::Down if self.cursor.y < self.document.len() => self.cursor.y += 1,
            Arrow::Left => {
                if self.cursor.x > 0 {
                    self.cursor.x -= 1
                } else if self.cursor.y > 0 {
                    self.cursor.y -= 1;
                    if let Some(row) = self.document.row(self.cursor.y) {
                        self.cursor.x = row.len();
                    }
                }
            }
            Arrow::Right => {
                if let Some(row) = self.document.row(self.cursor.y) {
                    let chars_len = row.len();
                    if self.cursor.x < chars_len {
                        self.cursor.x += 1
                    } else if self.cursor.x == chars_len {
                        self.cursor.y += 1;
                        self.cursor.x = 0;
                    }
                }
            }
            _ => {}
        }
        if let Some(r) = self.document.row(self.cursor.y) {
            if self.cursor.x > r.len() {
                self.cursor.x = r.len();
            }
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
            ESCAPE => {
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
            b'\r' => Ok(Key::Enter),
            BACKSPACE => Ok(Key::Backspace),
            DELETE_BIS => Ok(Key::Backspace),
            REFRESH_SCREEN => Ok(Key::Escape),
            SAVE => Ok(Key::Save),
            EXIT => Ok(Key::Exit),
            _ => Ok(Key::Char(b)),
        }
    }

    fn draw_rows(&self, buf: &mut String) {
        let width = self.screen.cols;
        let height = self.screen.rows;
        let rows = self.document.len();
        for y in 0..height {
            let file_row = y + self.row_offset;
            if file_row >= rows {
                if rows == 0 && y == height / 3 {
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
                if let Some(row) = self.document.row(file_row) {
                    let chars = &row.render(self.col_offset, self.col_offset + width);
                    buf.push_str(chars);
                }
            }

            buf.push_str(CLEAR_LINE_RIGHT_OF_CURSOR);
            buf.push_str("\r\n");
        }
    }

    fn draw_status_bar(&self, buf: &mut String) {
        buf.push_str(REVERSE_VIDEO);
        let mut filename = match &self.document.filename {
            Some(n) => n.clone(),
            None => String::from("[No Name]"),
        };
        filename.truncate(20);
        let left = format!("{} - {} lines", filename, self.document.len());
        let right = format!("{}/{}", self.cursor.y, self.document.len());
        let rlen = right.len();
        let mut len = cmp::min(left.len(), self.screen.cols);
        buf.push_str(&left);
        while len < self.screen.cols {
            if self.screen.cols - len == rlen {
                buf.push_str(&right);
                break;
            } else {
                buf.push_str(" ");
                len += 1;
            }
        }
        buf.push_str(RESET_FMT);
        buf.push_str("\r\n");
    }

    fn draw_message_bar(&self, buf: &mut String) {
        buf.push_str(CLEAR_LINE_RIGHT_OF_CURSOR);
        if let Some(message) = &self.message {
            if Instant::now() - message.time < Duration::new(5, 0) {
                let mut text = message.text.clone();
                text.truncate(self.screen.cols);
                buf.push_str(&text);
            }
        }
    }
}

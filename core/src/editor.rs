use super::{
    ansi_escape::*,
    document::Document,
    error::Error,
    traits::{Filer, Input, Output},
};
use std::{
    cmp,
    time::{Duration, Instant},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub enum Key {
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
    Char(char),
}

pub enum Page {
    Up,
    Down,
}

pub enum Arrow {
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
    fn new(text: impl Into<String>) -> Message {
        Message {
            text: text.into(),
            time: Instant::now(),
        }
    }
}

pub struct Editor<I: Input, O: Output, F: Filer> {
    input: I,
    output: O,
    filer: F,
    screen: Screen,
    cursor: Position,
    row_offset: usize,
    col_offset: usize,
    document: Document,
    message: Option<Message>,
    confirm: bool,
}

impl<I: Input, O: Output, F: Filer> Editor<I, O, F> {
    pub fn new(input: I, output: O, filer: F) -> Result<Self, Error> {
        if let Some((screen_rows, screen_cols)) = output.get_window_size() {
            Ok(Editor {
                input,
                output,
                filer,
                screen: Screen {
                    rows: screen_rows - 2,
                    cols: screen_cols,
                },
                cursor: Position { x: 0, y: 0 },
                row_offset: 0,
                col_offset: 0,
                document: Document::new(),
                message: Some(Message::new("HELP: Ctrl-S = save | Ctrl-Q = quit")),
                confirm: false,
            })
        } else {
            Err(Error::UnknownWindowSize)
        }
    }

    pub fn load(&mut self, filename: &String) -> Result<(), Error> {
        let file = self.filer.load(&filename)?;
        self.document = Document::open(filename.clone(), file);
        Ok(())
    }

    pub fn save(&mut self) {
        if self.document.filename.is_none() {
            self.document.filename = Some(String::from("TODO"));
        }
        let filename = self.document.filename.clone().unwrap();
        match self.filer.save(&filename, self.document.contents()) {
            Ok(_) => {
                self.message = Some(Message::new("File saved successfully."));
            }
            _ => {
                self.message = Some(Message::new("Error writing file!"));
            }
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            let quit = self.process_key_press()?;
            if quit == true {
                self.output.clear_screen()?;
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
        self.output.render(buf)?;

        let x = self
            .document
            .row(self.cursor.y)
            .map(|row| row.get_width(0, self.cursor.x - self.col_offset))
            .unwrap_or(0);
        self.output.move_cursor(Position {
            x: (x + 1),
            y: (self.cursor.y - self.row_offset) + 1,
        })?;
        self.output.render(SHOW_CURSOR.to_string())?;

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
        match self.input.wait_for_key() {
            Key::Escape => {}
            Key::Enter => {
                self.document.insert_newline(&self.cursor);
                self.move_cursor(&Arrow::Right);
            }
            Key::Backspace => {
                if self.cursor.x > 0 || self.cursor.y > 0 {
                    self.move_cursor(&Arrow::Left);
                    self.document.delete(&self.cursor);
                }
            }
            Key::Del => {
                self.document.delete(&self.cursor);
            }
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
            Key::Save => {
                self.save();
            }
            Key::Exit => {
                if self.document.is_dirty() && self.confirm == false {
                    self.confirm = true;
                    self.message = Some(Message::new(
                        "WARNING!!! File has unsaved changes. Press Ctrl-Q 1 more times to quit.",
                    ));
                    return Ok(false);
                } else {
                    return Ok(true);
                }
            }
            Key::Char(c) => {
                self.document.insert(c, &self.cursor);
                self.move_cursor(&Arrow::Right);
            }
            _ => {}
        }

        if self.confirm == true {
            self.confirm = false;
            self.message = None;
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
        let left = format!(
            "{} - {} lines {}",
            &self
                .document
                .filename
                .clone()
                .map(|mut n| {
                    n.truncate(20);
                    n
                })
                .unwrap_or(String::from("[No Name]")),
            self.document.len(),
            if self.document.is_dirty() {
                "(modified)"
            } else {
                ""
            }
        );
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

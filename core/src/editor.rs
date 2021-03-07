use super::{
    document::Document,
    error::Error,
    traits::{Filer, Input, Output},
};
use instant::Instant;
use std::{cmp, time::Duration};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub enum Key {
    Escape,
    Backspace,
    Del,
    Enter,
    Home,
    End,
    Command(Command),
    Page(Page),
    Arrow(Arrow),
    Char(char),
    CharUtf8(char),
    Unknown,
}

pub enum Command {
    Find,
    Undo,
    Redo,
    Save,
    Exit,
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

#[derive(Clone)]
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

pub enum SearchDirection {
    Forward,
    Backward,
}

enum Mode {
    Edit,
    Search,
    Save,
    Exit,
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
                message: Some(Message::new(
                    "HELP: Ctrl-S = save | Ctrl-Q = quit | Ctrl-F = find",
                )),
                confirm: false,
            })
        } else {
            Err(Error::UnknownWindowSize)
        }
    }

    pub fn load(&mut self, filename: &String) -> Result<&mut Self, Error> {
        let file = self.filer.load(&filename)?;
        self.document = Document::open(filename.clone(), file);
        Ok(self)
    }

    pub fn save(&mut self) -> Result<(), Error> {
        let filename = self.document.get_filename().unwrap_or(String::new());
        let res = self.filer.save(&filename, self.document.contents());
        if res.is_ok() {
            self.document.reset_dirty();
        }
        res
    }

    pub fn run(&mut self) -> Result<(), Error> {
        // loop {
            self.refresh_screen()?;
            match self.process_key_press()? {
                Mode::Edit => {}
                Mode::Search => {
                    self.search_prompt();
                }
                Mode::Save => {
                    self.save_prompt();
                }
                Mode::Exit => {
                    self.output.clear_screen();
                    // break; TODO
                }
            }
        // }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        self.scroll();
        self.document.update_highlights();

        let rows = self.draw_rows();
        let status_bar = self.draw_status_bar();
        let message_bar = self.draw_message_bar();

        let x = self
            .document
            .row(self.cursor.y)
            .map(|row| row.calc_width(0, self.cursor.x - self.col_offset))
            .unwrap_or(0);
        self.output.render_screen(
            rows,
            &status_bar,
            &message_bar,
            Position {
                x: (x + 1),
                y: (self.cursor.y - self.row_offset) + 1,
            },
        );
        self.output.flush()?;

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

    fn process_key_press(&mut self) -> Result<Mode, Error> {
        let mut pressed = true;
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
            Key::Command(command) => match command {
                Command::Find => {
                    return Ok(Mode::Search);
                }
                Command::Undo => {
                    self.document.undo();
                }
                Command::Redo => {
                    self.document.redo();
                }
                Command::Save => {
                    return Ok(Mode::Save);
                }
                Command::Exit => {
                    if self.document.is_dirty() && self.confirm == false {
                        self.confirm = true;
                        self.message = Some(Message::new(
                            "WARNING!!! File has unsaved changes. Press Ctrl-Q 1 more times to quit.",
                        ));
                        return Ok(Mode::Edit);
                    } else {
                        return Ok(Mode::Exit);
                    }
                }
            },
            Key::Char(c) | Key::CharUtf8(c) => {
                self.document.insert(c, &self.cursor);
                self.move_cursor(&Arrow::Right);
            }
            Key::Unknown => {
                pressed = false;
            }
        }

        if pressed == true && self.confirm == true {
            self.confirm = false;
            self.message = None;
        }
        Ok(Mode::Edit)
    }

    fn move_cursor(&mut self, key: &Arrow) {
        match key {
            Arrow::Up if self.cursor.y > 0 => {
                if let Some(row) = self.document.row(self.cursor.y) {
                    if let Some(row_next) = self.document.row(self.cursor.y - 1) {
                        self.cursor.x = row_next.calc_x(self.cursor.x, row);
                        self.cursor.y -= 1;
                    }
                }
            }
            Arrow::Down if self.cursor.y < self.document.len() => {
                if let Some(row) = self.document.row(self.cursor.y) {
                    if let Some(row_next) = self.document.row(self.cursor.y + 1) {
                        self.cursor.x = row_next.calc_x(self.cursor.x, row);
                        self.cursor.y += 1;
                    }
                }
            }
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

    fn save_prompt(&mut self) {
        if self.document.get_filename().is_none() {
            let filename = self.prompt("Save as", "ESC to cancel", |_, _, _| {});
            if filename.is_none() {
                self.message = Some(Message::new("Save aborted"));
                return;
            }
            self.document.set_filename(filename);
        }
        self.message = match self.save() {
            Ok(_) => Some(Message::new("File saved successfully.")),
            Err(_) => Some(Message::new("Error writing file!")),
        }
    }

    fn search_prompt(&mut self) {
        let cursor = self.cursor.clone();
        let mut direction = SearchDirection::Forward;
        let res = self.prompt("Search", "Use ESC/Arrows/Enter", |editor, key, query| {
            let mut moved = false;
            match key {
                Key::Arrow(Arrow::Left) | Key::Arrow(Arrow::Up) => {
                    direction = SearchDirection::Backward;
                    editor.move_cursor(&Arrow::Left);
                    moved = true;
                }
                Key::Arrow(Arrow::Right) | Key::Arrow(Arrow::Down) => {
                    direction = SearchDirection::Forward;
                    editor.move_cursor(&Arrow::Right);
                    moved = true;
                }
                _ => {
                    direction = SearchDirection::Forward;
                }
            }

            if let Some(pos) = editor.document.find(&query, &editor.cursor, &direction) {
                editor.cursor = pos;
                editor.scroll();
            } else if moved == true {
                match direction {
                    SearchDirection::Forward => editor.move_cursor(&Arrow::Left),
                    SearchDirection::Backward => editor.move_cursor(&Arrow::Right),
                }
            }
        });
        if res.is_none() {
            self.cursor = cursor;
            self.scroll();
        }
    }

    fn prompt<C>(&mut self, desc1: &str, desc2: &str, mut cb: C) -> Option<String>
    where
        C: FnMut(&mut Self, Key, &String),
    {
        let mut message = String::new();
        loop {
            self.message = Some(Message::new(format!("{}: {} ({})", desc1, message, desc2)));
            if self.refresh_screen().is_err() {
                return None;
            }
            let key = self.input.wait_for_key();
            match key {
                Key::Escape => {
                    self.message = None;
                    return None;
                }
                Key::Del | Key::Backspace => {
                    message.pop();
                }
                Key::Enter => {
                    if message.len() != 0 {
                        self.message = None;
                        return Some(message);
                    }
                }
                Key::Char(c) | Key::CharUtf8(c) => {
                    message.push(c);
                }
                _ => {}
            }
            cb(self, key, &message);
        }
    }

    fn draw_rows(&mut self) -> Vec<String> {
        let mut vec = Vec::new();
        let width = self.screen.cols;
        let height = self.screen.rows;
        let rows = self.document.len();
        for y in 0..height {
            let mut buf = String::new();
            let r_index = y + self.row_offset;
            if r_index >= rows {
                if rows == 0 && y == height / 3 {
                    let message = create_welcome_message(width);
                    buf.push_str(&message);
                } else {
                    buf.push_str("~");
                }
            } else {
                buf.push_str(&self.document.render_row(
                    r_index,
                    self.col_offset,
                    self.col_offset + width,
                ));
            }
            vec.push(buf);
        }
        vec
    }

    fn draw_status_bar(&self) -> String {
        let mut buf = String::new();
        let left = format!(
            "{} - {} lines {}",
            &self
                .document
                .get_filename()
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
        let right = format!(
            "{} | {}/{}",
            self.document.language.name(),
            self.cursor.y,
            self.document.len()
        );
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
        buf
    }

    fn draw_message_bar(&self) -> String {
        let mut buf = String::new();
        if let Some(message) = &self.message {
            if Instant::now() - message.time < Duration::new(5, 0) {
                let mut text = message.text.clone();
                text.truncate(self.screen.cols);
                buf.push_str(&text);
            }
        }
        buf
    }
}

fn create_welcome_message(width: usize) -> String {
    let message = format!("Kilo editor -- version {}", VERSION);
    let padding = width.saturating_sub(message.len()) / 2;
    let spaces = " ".repeat(padding.saturating_sub(1));
    let mut message = format!("~{}{}", spaces, message);
    message.truncate(width);
    message
}

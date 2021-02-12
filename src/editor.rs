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
const UP: u8 = b'w';
const DOWN: u8 = b's';
const LEFT: u8 = b'a';
const RIGHT: u8 = b'd';

enum Key {
    Exit,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Todo,
}

struct Position {
    x: usize,
    y: usize,
}

pub struct Editor {
    input: StdinRaw,
    screen_rows: usize,
    screen_cols: usize,
    cursor: Position,
}

impl Editor {
    pub fn new(stdin: StdinRaw) -> Result<Self, Error> {
        if let Some((screen_rows, screen_cols)) = get_window_size() {
            Ok(Editor {
                input: stdin,
                screen_rows,
                screen_cols,
                cursor: Position { x: 0, y: 0 },
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
        let b = self.input.read()?;
        match self.decode_sequence(b) {
            Key::ArrowUp if self.cursor.y > 0 => self.cursor.y -= 1,
            Key::ArrowDown if self.cursor.y < self.screen_rows - 1 => self.cursor.y += 1,
            Key::ArrowLeft if self.cursor.x > 0 => self.cursor.x -= 1,
            Key::ArrowRight if self.cursor.x < self.screen_cols - 1 => self.cursor.x += 1,
            Key::Exit => return Ok(true),
            _ => {}
        }
        Ok(false)
    }

    fn decode_sequence(&self, b: u8) -> Key {
        match b {
            UP => Key::ArrowUp,
            DOWN => Key::ArrowDown,
            LEFT => Key::ArrowLeft,
            RIGHT => Key::ArrowRight,
            EXIT => Key::Exit,
            _ => Key::Todo,
        }
    }

    fn draw_rows(&self, buf: &mut String) {
        let width = self.screen_cols;
        let height = self.screen_rows;
        for y in 0..height {
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
            buf.push_str(CLEAR_LINE_RIGHT_OF_CURSOR);
            if y < height - 1 {
                buf.push_str("\r\n");
            }
        }
    }
}

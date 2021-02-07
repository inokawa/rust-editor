use super::{error::Error, input_unix::StdinRaw, output_unix::get_window_size};
use std::io::{self, Write};

const fn ctrl(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}
const EXIT: u8 = ctrl('q');

pub struct Editor {
    input: StdinRaw,
    screen_rows: usize,
    screen_cols: usize,
}

impl Editor {
    pub fn new(stdin: StdinRaw) -> Result<Self, Error> {
        if let Some((screen_rows, screen_cols)) = get_window_size() {
            Ok(Editor {
                input: stdin,
                screen_rows,
                screen_cols,
            })
        } else {
            Err(Error::Init)
        }
    }

    pub fn run(&self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            let quit = self.process_key_press()?;
            if quit == true {
                let mut buf = String::new();
                buf.push_str("\x1b[2J");
                buf.push_str("\x1b[H");
                print!("{}", buf);
                break;
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        let mut buf = String::new();
        buf.push_str("\x1b[?25l");
        buf.push_str("\x1b[H");

        self.draw_rows(&mut buf);

        buf.push_str("\x1b[H");
        buf.push_str("\x1b[?25h");

        print!("{}", buf);
        Ok(())
    }

    fn process_key_press(&self) -> Result<bool, Error> {
        let b = self.input.read()?;
        let c = b as char;
        print!("{} ({:?})\r\n", c, b);
        match b {
            EXIT => return Ok(true),
            _ => {}
        }
        Ok(false)
    }

    fn draw_rows(&self, buf: &mut String) {
        for y in 0..self.screen_rows {
            buf.push_str("~");
            buf.push_str("\x1b[K");
            if y < self.screen_rows - 1 {
                buf.push_str("\r\n");
            }
        }
    }
}

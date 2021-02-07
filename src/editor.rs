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
    pub fn new(stdin: StdinRaw) -> Self {
        let (screen_rows, screen_cols) = get_window_size().unwrap();
        Editor {
            input: stdin,
            screen_rows,
            screen_cols,
        }
    }

    pub fn run(&self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            let quit = self.process_key_press()?;
            if quit == true {
                self.clear_screen()?;
                break;
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        self.clear_screen()?;
        self.draw_rows();
        print!("\x1b[H");
        Ok(())
    }

    fn process_key_press(&self) -> Result<bool, Error> {
        for b in self.input.bytes() {
            let b = b?;
            let c = b as char;
            print!("{} ({:?})\r\n", c, b);
            match b {
                EXIT => return Ok(true),
                _ => {}
            }
        }
        Ok(false)
    }

    fn clear_screen(&self) -> Result<(), io::Error> {
        // Clear screen
        print!("\x1b[2J");
        // Move cursor to left top
        print!("\x1b[H");
        io::stdout().flush()
    }

    fn draw_rows(&self) {
        for _ in 0..=self.screen_rows {
            print!("~\r\n");
        }
    }
}

use core::{Ansi, Error, Output, Position};
use libc::*;
use std::{
    io::{self, Write},
    mem,
};

pub struct Stdout {}

impl Output for Stdout {
    fn new() -> Self {
        Stdout {}
    }

    fn write(&self, text: &str) {
        print!("{}", text);
    }

    fn flush(&self) -> Result<(), Error> {
        io::stdout().flush()?;
        Ok(())
    }

    fn render_screen(&self, rows: Vec<String>, status_bar: &str, message_bar: &str, pos: Position) {
        let buf = self.render_screen_wrap(rows, status_bar, message_bar, pos);
        self.write(&buf);
    }

    fn clear_screen(&self) {
        let buf = self.clear_screen_wrap();
        self.write(&buf);
    }

    fn get_window_size(&self) -> Option<(usize, usize)> {
        let mut ws: winsize = unsafe { mem::zeroed() };
        if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) } == -1 {
            None
        } else {
            Some((ws.ws_row as usize, ws.ws_col as usize))
        }
    }
}

impl Ansi for Stdout {}

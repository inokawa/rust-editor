use core::{ansi_escape::*, Error, Output, Position};
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

    fn render(&self, text: String) -> Result<(), Error> {
        print!("{}", text);
        io::stdout().flush()?;
        Ok(())
    }

    fn move_cursor(&self, pos: Position) -> Result<(), Error> {
        let mut buf = String::new();
        buf.push_str(MOVE_CURSOR_TO_START);
        buf.push_str(&format!("\x1b[{};{}H", pos.y, pos.x));
        self.render(buf)
    }

    fn clear_screen(&self) -> Result<(), Error> {
        let mut buf = String::new();
        buf.push_str(CLEAR_SCREEN);
        buf.push_str(MOVE_CURSOR_TO_START);
        self.render(buf)
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

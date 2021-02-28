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

    fn render(&self, text: &str) {
        print!("{}", text);
    }

    fn flush(&self) -> Result<(), Error> {
        io::stdout().flush()?;
        Ok(())
    }

    fn render_screen(&self, text: &str, pos: Position) {
        let mut buf = String::new();
        buf.push_str(HIDE_CURSOR);
        buf.push_str(MOVE_CURSOR_TO_START);
        buf.push_str(text);
        buf.push_str(&format!("\x1b[{};{}H", pos.y, pos.x));
        buf.push_str(SHOW_CURSOR);
        self.render(&buf);
    }

    fn clear_screen(&self) {
        let mut buf = String::new();
        buf.push_str(CLEAR_SCREEN);
        buf.push_str(MOVE_CURSOR_TO_START);
        self.render(&buf);
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

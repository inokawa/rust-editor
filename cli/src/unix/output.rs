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

    fn render_screen(&self, rows: Vec<String>, status_bar: &str, message_bar: &str, pos: Position) {
        let mut buf = String::new();
        buf.push_str(HIDE_CURSOR);
        buf.push_str(MOVE_CURSOR_TO_START);
        rows.iter().for_each(|r| {
            buf.push_str(&r);
            buf.push_str(CLEAR_LINE_RIGHT_OF_CURSOR);
            buf.push_str("\r\n");
        });
        buf.push_str(REVERSE_VIDEO);
        buf.push_str(status_bar);
        buf.push_str(RESET_FMT);
        buf.push_str("\r\n");
        buf.push_str(CLEAR_LINE_RIGHT_OF_CURSOR);
        buf.push_str(message_bar);
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

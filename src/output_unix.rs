use super::{error::Error, output_trait::Output};
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

    fn get_window_size(&self) -> Option<(usize, usize)> {
        let mut ws: winsize = unsafe { mem::zeroed() };
        if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) } == -1 {
            None
        } else {
            Some((ws.ws_row as usize, ws.ws_col as usize))
        }
    }
}

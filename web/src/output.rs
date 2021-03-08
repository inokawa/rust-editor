use super::xterm;
use core::{Ansi, Error, Output, Position};

pub struct WebOutput {}

impl Output for WebOutput {
    fn new() -> Self {
        WebOutput {}
    }

    fn write(&self, text: &str) {
        xterm::xterm_write(&text);
    }

    fn flush(&self) -> Result<(), Error> {
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
        Some((40, 100))
        // if let Some(win) = window() {
        //     match (win.inner_width(), win.inner_height()) {
        //         (Ok(w), Ok(h)) => match (w.as_f64(), h.as_f64()) {
        //             (Some(w), Some(h)) => Some((w as usize, h as usize)),
        //             (_, _) => None,
        //         },
        //         (_, _) => None,
        //     }
        // } else {
        //     None
        // }
    }
}

impl Ansi for WebOutput {}

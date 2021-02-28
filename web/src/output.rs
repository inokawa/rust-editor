use core::{Error, Output, Position};

pub struct WebOutput {}

impl Output for WebOutput {
    fn new() -> Self {
        WebOutput {}
    }

    fn move_cursor(&self, pos: Position) {}

    fn clear_screen(&self) {}

    fn render(&self, text: &str) {}

    fn flush(&self) -> Result<(), Error> {
        Ok(())
    }

    fn hide_cursor(&self) {}

    fn show_cursor(&self) {}

    fn get_window_size(&self) -> Option<(usize, usize)> {
        Some((123, 456))
    }
}

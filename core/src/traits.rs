use super::{
    editor::{Key, Position},
    error::Error,
};

pub trait Input {
    fn new() -> Result<Self, Error>
    where
        Self: Sized;
    fn wait_for_key(&self) -> Key;
}

pub trait Output {
    fn new() -> Self;
    fn move_cursor(&self, pos: Position) -> ();
    fn clear_screen(&self) -> ();
    fn render(&self, text: &str) -> ();
    fn flush(&self) -> Result<(), Error>;
    fn hide_cursor(&self) -> ();
    fn show_cursor(&self) -> ();
    fn get_window_size(&self) -> Option<(usize, usize)>;
}

pub trait Filer {
    fn new() -> Self;
    fn load(&self, filename: &String) -> Result<String, Error>;
    fn save(&self, filename: &String, contents: Vec<String>) -> Result<(), Error>;
}

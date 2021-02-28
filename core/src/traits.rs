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
    fn render_screen(
        &self,
        rows: Vec<String>,
        status_bar: &str,
        message_bar: &str,
        pos: Position,
    ) -> ();
    fn clear_screen(&self) -> ();
    fn render(&self, text: &str) -> ();
    fn flush(&self) -> Result<(), Error>;
    fn get_window_size(&self) -> Option<(usize, usize)>;
}

pub trait Filer {
    fn new() -> Self;
    fn load(&self, filename: &String) -> Result<String, Error>;
    fn save(&self, filename: &String, contents: Vec<String>) -> Result<(), Error>;
}

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
    fn clear_screen(&self) -> Result<(), Error>;
    fn move_cursor(&self, pos: Position) -> Result<(), Error>;
    fn render(&self, text: String) -> Result<(), Error>;
    fn get_window_size(&self) -> Option<(usize, usize)>;
}

pub trait Filer {
    fn new() -> Self;
    fn load(&self, filename: &String) -> Result<String, Error>;
    fn save(&self, filename: &String, contents: Vec<String>) -> Result<(), Error>;
}

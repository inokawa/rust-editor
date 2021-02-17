use super::{editor::Key, error::Error};

pub trait Input {
    fn new() -> Result<Self, Error>
    where
        Self: Sized;
    fn wait_for_key(&self) -> Key;
}

pub trait Output {
    fn new() -> Self;
    fn render(&self, text: String) -> Result<(), Error>;
    fn get_window_size(&self) -> Option<(usize, usize)>;
}

pub trait Filer {
    fn new() -> Self;
    fn load(&self, filename: &String) -> Result<String, Error>;
    fn save(&self, filename: &String, contents: Vec<String>) -> Result<(), Error>;
}

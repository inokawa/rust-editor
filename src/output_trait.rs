use super::error::Error;

pub trait Output {
    fn new() -> Self;
    fn render(&self, text: String) -> Result<(), Error>;
    fn get_window_size(&self) -> Option<(usize, usize)>;
}

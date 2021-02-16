use super::error::Error;

pub trait Input {
    fn new() -> Result<Self, Error>
    where
        Self: Sized;
    fn read(&self) -> Option<u8>;
}

use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    UnknownWindowSize,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

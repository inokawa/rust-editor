use super::error::Error;
use std::fs;

pub struct Filer {}

impl Filer {
    pub fn new() -> Self {
        Filer {}
    }

    pub fn load(&self, filename: &String) -> Result<String, Error> {
        let file = fs::read_to_string(&filename)?;
        Ok(file)
    }
}

use super::{document::Document, error::Error};
use std::fs;

pub struct Filer {}

impl Filer {
    pub fn new() -> Self {
        Filer {}
    }

    pub fn load(&self, filename: &String) -> Result<Document, Error> {
        let file = fs::read_to_string(&filename)?;
        Ok(Document::open(filename.clone(), file))
    }
}

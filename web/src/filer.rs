use core::{Error, Filer};

pub struct WebFile {}

impl Filer for WebFile {
    fn new() -> Self {
        WebFile {}
    }

    fn load(&self, filename: &String) -> Result<String, Error> {
        Ok("TODO".to_string())
    }

    fn save(&self, filename: &String, contents: Vec<String>) -> Result<(), Error> {
        Ok(())
    }
}

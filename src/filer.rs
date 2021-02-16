use super::error::Error;
use std::{fs, io::Write};

pub struct Filer {}

impl Filer {
    pub fn new() -> Self {
        Filer {}
    }

    pub fn load(&self, filename: &String) -> Result<String, Error> {
        let file = fs::read_to_string(&filename)?;
        Ok(file)
    }

    pub fn save(&self, filename: &String, contents: Vec<String>) -> Result<(), Error> {
        let mut file = fs::File::create(filename)?;
        for row in &contents {
            file.write_all(row.as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(())
    }
}

use core::{Error, Filer};
use std::{fs, io::Write};

pub struct Fs {}

impl Filer for Fs {
    fn new() -> Self {
        Fs {}
    }

    fn load(&self, filename: &String) -> Result<String, Error> {
        let file = fs::read_to_string(&filename)?;
        Ok(file)
    }

    fn save(&self, filename: &String, contents: Vec<String>) -> Result<(), Error> {
        let mut file = fs::File::create(filename)?;
        for row in &contents {
            file.write_all(row.as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(())
    }
}

use rust_editor::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Error> {
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                println!("{}", c);
            }
            Err(e) => return Err(Error::IO(e)),
        }
    }
    Ok(())
}

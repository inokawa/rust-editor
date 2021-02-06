use rust_editor::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Error> {
    for b in io::stdin().bytes() {
        let b = b?;
        let c = b as char;
        println!("{}", c);
    }
    Ok(())
}

use std::io::{self, Read};

fn main() {
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                println!("{}", c);
            }
            Err(e) => {
                panic!(e);
            }
        }
    }
    println!("Hello, world!");
}

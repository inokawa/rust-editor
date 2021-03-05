use cli::{Fs, StdinRaw, Stdout};
use core::{Editor, Error, Filer, Input, Output};
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let input = StdinRaw::new()?;
    if let Some(filename) = args.get(1) {
        Editor::new(input, Stdout::new(), Fs::new())?
            .load(filename)?
            .run()
    } else {
        Editor::new(input, Stdout::new(), Fs::new())?.run()
    }
}

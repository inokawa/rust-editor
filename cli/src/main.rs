use cli::{Fs, StdinRaw, Stdout};
use core::{Editor, Error, Filer, Input, Output};
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let mut editor = Editor::new(StdinRaw::new()?, Stdout::new(), Fs::new())?;
    if let Some(filename) = args.get(1) {
        editor.load(filename)?;
    }
    loop {
        let quit = editor.run()?;
        if quit {
            break;
        }
    }
    Ok(())
}

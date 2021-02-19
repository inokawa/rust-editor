use core::{
    editor::Editor,
    error::Error,
    filer_unix::Fs,
    input_unix::StdinRaw,
    output_unix::Stdout,
    traits::{Filer, Input, Output},
};
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let input = StdinRaw::new()?;
    if let Some(filename) = args.get(1) {
        let mut editor = Editor::new(input, Stdout::new(), Fs::new())?;
        editor.load(filename)?;
        editor.run()
    } else {
        Editor::new(input, Stdout::new(), Fs::new())?.run()
    }
}

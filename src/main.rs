use rust_editor::{document::Document, editor::Editor, error::Error, input_unix::StdinRaw};
use std::{env, fs};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let input = StdinRaw::new()?;
    if let Some(filename) = args.get(1) {
        let file = fs::read_to_string(filename)?;
        Editor::new(input, Document::open(filename.clone(), file))?.run()
    } else {
        Editor::new(input, Document::new())?.run()
    }
}

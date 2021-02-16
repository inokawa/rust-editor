use rust_editor::{editor::Editor, error::Error, filer::Filer, input_unix::StdinRaw};
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let input = StdinRaw::new()?;
    if let Some(filename) = args.get(1) {
        let mut editor = Editor::new(input, Filer::new())?;
        editor.load(filename)?;
        editor.run()
    } else {
        Editor::new(input, Filer::new())?.run()
    }
}

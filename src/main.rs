use rust_editor::{
    editor::Editor, error::Error, filer::Filer, input_trait::Input, input_unix::StdinRaw,
    output_trait::Output, output_unix::Stdout,
};
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let input = StdinRaw::new()?;
    if let Some(filename) = args.get(1) {
        let mut editor = Editor::new(input, Stdout::new(), Filer::new())?;
        editor.load(filename)?;
        editor.run()
    } else {
        Editor::new(input, Stdout::new(), Filer::new())?.run()
    }
}

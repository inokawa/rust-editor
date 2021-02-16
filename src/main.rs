use rust_editor::{
    document::Document, editor::Editor, error::Error, filer::Filer, input_unix::StdinRaw,
};
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let input = StdinRaw::new()?;
    if let Some(filename) = args.get(1) {
        let filer = Filer::new();
        let doc = filer.load(filename)?;
        Editor::new(input, doc)?.run()
    } else {
        Editor::new(input, Document::new())?.run()
    }
}

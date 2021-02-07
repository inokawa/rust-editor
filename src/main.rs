use rust_editor::{editor::Editor, error::Error, input_unix::StdinRaw};

fn main() -> Result<(), Error> {
    let input = StdinRaw::new()?;
    Editor::new(input).run()
}

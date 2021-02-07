use rust_editor::{error::Error, input_unix::StdinRaw};

fn main() -> Result<(), Error> {
    let input = StdinRaw::new()?;
    'app: loop {
        for b in input.bytes() {
            let b = b?;
            let c = b as char;
            print!("{} ({:?})\r\n", c, b);
            if c == 'q' {
                print!("quit\r\n");
                break 'app;
            }
        }
    }
    Ok(())
}

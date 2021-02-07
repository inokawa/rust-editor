use rust_editor::{error::Error, input_unix::StdinRaw};

const fn ctrl(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}
const EXIT: u8 = ctrl('q');

fn process_key_press(input: &StdinRaw) -> Result<bool, Error> {
    for b in input.bytes() {
        let b = b?;
        let c = b as char;
        print!("{} ({:?})\r\n", c, b);
        match b {
            EXIT => return Ok(true),
            _ => {}
        }
    }
    Ok(false)
}

fn main() -> Result<(), Error> {
    let input = StdinRaw::new()?;
    loop {
        let quit = process_key_press(&input)?;
        if quit == true {
            print!("quit\r\n");
            break;
        }
    }
    Ok(())
}

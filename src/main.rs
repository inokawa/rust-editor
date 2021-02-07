use rust_editor::{error::Error, input_unix::StdinRaw};

fn to_ctrl_byte(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}

fn process_key_press(input: &StdinRaw) -> Result<bool, Error> {
    for b in input.bytes() {
        let b = b?;
        let c = b as char;
        print!("{} ({:?})\r\n", c, b);
        if b == to_ctrl_byte('q') {
            return Ok(true);
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

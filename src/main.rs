use rust_editor::{error::Error, input_unix::StdinRaw};
use std::io::{self, Write};

const fn ctrl(c: char) -> u8 {
    (c as u8) & 0b0001_1111
}
const EXIT: u8 = ctrl('q');

fn clear_screen() -> Result<(), io::Error> {
    // Clear screen
    print!("\x1b[2J");
    // Move cursor to left top
    print!("\x1b[H");
    io::stdout().flush()
}

fn draw_rows() {
    for _ in 0..=24 {
        print!("~\r\n");
    }
}

fn refresh_screen() -> Result<(), Error> {
    clear_screen()?;
    draw_rows();
    print!("\x1b[H");
    Ok(())
}

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
        refresh_screen()?;
        let quit = process_key_press(&input)?;
        if quit == true {
            clear_screen()?;
            break;
        }
    }
    Ok(())
}

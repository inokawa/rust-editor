use super::{ansi_escape::*, error::Error, traits::Input};
use libc::{
    tcgetattr, tcsetattr, termios, BRKINT, CS8, ECHO, ICANON, ICRNL, IEXTEN, INPCK, ISIG, ISTRIP,
    IXON, OPOST, STDIN_FILENO, TCSAFLUSH, VMIN, VTIME,
};
use std::io::{self, Read};

#[cfg(unix)]
pub struct StdinRaw {
    orig: termios,
}

impl Input for StdinRaw {
    fn new() -> Result<Self, Error> {
        let mut term = termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_cc: [0u8; 20],
            c_ispeed: 0,
            c_ospeed: 0,
        };
        unsafe { tcgetattr(STDIN_FILENO, &mut term) };

        let orig = term;

        // Set terminal raw mode. Disable echo back, canonical mode, signals (SIGINT, SIGTSTP) and Ctrl+V.
        term.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
        // Disable control flow mode (Ctrl+Q/Ctrl+S) and CR-to-NL translation
        term.c_iflag &= !(IXON | ICRNL | BRKINT | INPCK | ISTRIP);
        // Disable output processing such as \n to \r\n translation
        term.c_oflag &= !OPOST;
        // Ensure character size is 8bits
        term.c_cflag |= CS8;
        // Do not wait for next byte with blocking since reading 0 byte is permitted
        term.c_cc[VMIN] = 0;
        // Set read timeout to 1/10 second it enables 100ms timeout on read()
        term.c_cc[VTIME] = 1;
        // Apply terminal configurations
        unsafe { tcsetattr(STDIN_FILENO, TCSAFLUSH, &term) };

        print!("{}", SMCUP);

        Ok(StdinRaw { orig })
    }

    fn read(&self) -> Option<u8> {
        if let Some(b) = io::stdin().bytes().next() {
            b.map(|b| Some(b)).unwrap_or(None)
        } else {
            None
        }
    }
}

impl Drop for StdinRaw {
    fn drop(&mut self) {
        print!("{}", RMCUP);

        unsafe { tcsetattr(STDIN_FILENO, TCSAFLUSH, &self.orig) };
    }
}

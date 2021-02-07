use super::error::Error;
use std::{
    io::{self, Read},
    os::unix::io::AsRawFd,
};
use termios::*;

#[cfg(unix)]
pub struct StdinRaw {
    fd: i32,
    orig: Termios,
}

impl StdinRaw {
    pub fn new() -> Result<Self, Error> {
        let fd = io::stdin().as_raw_fd();
        let mut termios = Termios::from_fd(fd)?;
        let orig = termios;

        // Set terminal raw mode. Disable echo back, canonical mode, signals (SIGINT, SIGTSTP) and Ctrl+V.
        termios.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
        // Disable control flow mode (Ctrl+Q/Ctrl+S) and CR-to-NL translation
        termios.c_iflag &= !(IXON | ICRNL | BRKINT | INPCK | ISTRIP);
        // Disable output processing such as \n to \r\n translation
        termios.c_oflag &= !OPOST;
        // Ensure character size is 8bits
        termios.c_cflag |= CS8;
        // Do not wait for next byte with blocking since reading 0 byte is permitted
        termios.c_cc[VMIN] = 0;
        // Set read timeout to 1/10 second it enables 100ms timeout on read()
        termios.c_cc[VTIME] = 1;
        // Apply terminal configurations
        tcsetattr(fd, TCSAFLUSH, &termios)?;

        Ok(StdinRaw { fd, orig })
    }

    pub fn read(&self) -> Result<u8, std::io::Error> {
        loop {
            if let Some(b) = io::stdin().bytes().next() {
                return b;
            }
        }
    }
}

impl Drop for StdinRaw {
    fn drop(&mut self) {
        tcsetattr(self.fd, TCSAFLUSH, &self.orig).unwrap();
    }
}

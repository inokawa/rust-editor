use libc::*;
use std::mem;

pub fn get_window_size() -> Option<(usize, usize)> {
    let mut ws: winsize = unsafe { mem::zeroed() };
    if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) } == -1 {
        None
    } else {
        Some((ws.ws_row as usize, ws.ws_col as usize))
    }
}

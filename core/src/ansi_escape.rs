use super::Position;

const CLEAR_SCREEN: &str = "\x1b[2J";
const CLEAR_LINE_RIGHT_OF_CURSOR: &str = "\x1b[K";
const MOVE_CURSOR_TO_START: &str = "\x1b[H";
const HIDE_CURSOR: &str = "\x1b[?25l";
const SHOW_CURSOR: &str = "\x1b[?25h";
const RESET_FMT: &str = "\x1b[m";
pub const SMCUP: &str = "\x1b[?1049h";
pub const RMCUP: &str = "\x1b[?1049l";

pub const DEFAULT: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const REVERSE_VIDEO: &str = "\x1b[7m";
pub const COLOR_BLACK: &str = "\x1b[30m";
pub const COLOR_RED: &str = "\x1b[31m";
pub const COLOR_GREEN: &str = "\x1b[32m";
pub const COLOR_YELLOW: &str = "\x1b[33m";
pub const COLOR_BLUE: &str = "\x1b[34m";
pub const COLOR_MAGENTA: &str = "\x1b[35m";
pub const COLOR_CYAN: &str = "\x1b[36m";
pub const COLOR_WHITE: &str = "\x1b[37m";
pub const COLOR_DEFAULT: &str = "\x1b[39m";
pub const BACKGROUND_COLOR_BLACK: &str = "\x1b[40m";
pub const BACKGROUND_COLOR_RED: &str = "\x1b[41m";
pub const BACKGROUND_COLOR_GREEN: &str = "\x1b[42m";
pub const BACKGROUND_COLOR_YELLOW: &str = "\x1b[43m";
pub const BACKGROUND_COLOR_BLUE: &str = "\x1b[44m";
pub const BACKGROUND_COLOR_MAGENTA: &str = "\x1b[45m";
pub const BACKGROUND_COLOR_CYAN: &str = "\x1b[46m";
pub const BACKGROUND_COLOR_GRAY: &str = "\x1b[47m";
pub const BACKGROUND_COLOR_DEFAULT: &str = "\x1b[49m";

pub trait Ansi {
    fn render_screen_wrap(
        &self,
        rows: Vec<String>,
        status_bar: &str,
        message_bar: &str,
        pos: Position,
    ) -> String {
        let mut buf = String::new();
        buf.push_str(HIDE_CURSOR);
        buf.push_str(MOVE_CURSOR_TO_START);
        rows.iter().for_each(|r| {
            buf.push_str(&r);
            buf.push_str(CLEAR_LINE_RIGHT_OF_CURSOR);
            buf.push_str("\r\n");
        });
        buf.push_str(REVERSE_VIDEO);
        buf.push_str(status_bar);
        buf.push_str(RESET_FMT);
        buf.push_str("\r\n");
        buf.push_str(CLEAR_LINE_RIGHT_OF_CURSOR);
        buf.push_str(message_bar);
        buf.push_str(&format!("\x1b[{};{}H", pos.y, pos.x));
        buf.push_str(SHOW_CURSOR);
        buf
    }

    fn clear_screen_wrap(&self) -> String {
        let mut buf = String::new();
        buf.push_str(CLEAR_SCREEN);
        buf.push_str(MOVE_CURSOR_TO_START);
        buf
    }
}

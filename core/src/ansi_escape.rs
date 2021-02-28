use super::Position;

const CLEAR_SCREEN: &str = "\x1b[2J";
const CLEAR_LINE_RIGHT_OF_CURSOR: &str = "\x1b[K";
const MOVE_CURSOR_TO_START: &str = "\x1b[H";
const HIDE_CURSOR: &str = "\x1b[?25l";
const SHOW_CURSOR: &str = "\x1b[?25h";
const REVERSE_VIDEO: &str = "\x1b[7m";
const RESET_FMT: &str = "\x1b[m";
pub const SMCUP: &str = "\x1b[?1049h";
pub const RMCUP: &str = "\x1b[?1049l";

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

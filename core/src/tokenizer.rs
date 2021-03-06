use super::ansi_escape::*;

#[derive(Clone)]
pub struct Token {
    pub index: usize,
    pub highlight: Highlight,
}

#[derive(Clone)]
pub enum Highlight {
    Number,
    None,
}

impl Highlight {
    pub fn color(&self) -> &str {
        match self {
            Highlight::Number => COLOR_RED,
            Highlight::None => COLOR_WHITE,
        }
    }
}

pub fn matcher(s: &str) -> Option<Highlight> {
    match s {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => Some(Highlight::Number),
        _ => None,
    }
}

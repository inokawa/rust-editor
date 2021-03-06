use super::ansi_escape::*;

#[derive(Clone)]
pub struct Token {
    pub index: usize,
    pub highlight: Highlight,
}

#[derive(Clone, PartialEq)]
pub enum Highlight {
    String,
    Number,
    None,
}

impl Highlight {
    pub fn color(&self) -> &str {
        match self {
            Highlight::String => COLOR_MAGENTA,
            Highlight::Number => COLOR_RED,
            Highlight::None => COLOR_DEFAULT,
        }
    }
}

pub fn is_digit(s: &str) -> bool {
    match s {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => true,
        _ => false,
    }
}

pub fn is_separator(s: &str) -> bool {
    s == " " || s == "\0" || ",.()+-/*=~%<>[];".contains(s)
}

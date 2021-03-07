use super::ansi_escape::*;

#[derive(Clone)]
pub struct Token {
    pub index: usize,
    pub highlight: Highlight,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Highlight {
    Comment,
    String,
    Number,
    None,
}

impl Highlight {
    pub fn color(&self) -> &str {
        match self {
            Highlight::Comment => COLOR_CYAN,
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

pub enum Language {
    C,
    Unknown,
}

impl Language {
    pub fn detect(filename: &str) -> Language {
        for ext in Language::C.exts() {
            if filename.ends_with(ext) {
                return Language::C;
            }
        }
        Language::Unknown
    }

    pub fn name(&self) -> &str {
        match self {
            Language::C => "C",
            Language::Unknown => "no ft",
        }
    }

    pub fn exts(&self) -> &'static [&'static str] {
        match self {
            Language::C => &[".c", ".h", ".cpp"],
            Language::Unknown => &[],
        }
    }

    pub fn flags(&self) -> &'static [&'static Highlight] {
        match self {
            Language::C => &[&Highlight::String, &Highlight::Number],
            Language::Unknown => &[],
        }
    }
}

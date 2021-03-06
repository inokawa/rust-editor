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
    pub fn color(&self) -> u8 {
        match self {
            Highlight::Number => 31,
            Highlight::None => 37,
        }
    }
}

pub fn matcher(s: &str) -> Option<Highlight> {
    match s {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => Some(Highlight::Number),
        _ => None,
    }
}

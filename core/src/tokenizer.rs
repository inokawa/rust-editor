#[derive(Clone)]
pub struct Token {
    pub index: usize,
    pub highlight: Highlight,
}

#[derive(Clone)]
pub enum Highlight {
    None,
    Number,
}

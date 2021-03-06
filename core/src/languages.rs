use super::tokenizer::Highlight;

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

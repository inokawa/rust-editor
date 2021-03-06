pub enum Language {
    C,
    Unknown,
}

#[derive(PartialEq)]
pub enum Flag {
    Number,
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

    pub fn flags(&self) -> &'static [&'static Flag] {
        match self {
            Language::C => &[&Flag::Number],
            Language::Unknown => &[],
        }
    }
}

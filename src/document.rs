use std::cmp;

const TAB_STOP: usize = 4;

pub struct Document {
    pub rows: Vec<Row>,
}

impl Document {
    pub fn new() -> Self {
        Document { rows: vec![] }
    }

    pub fn open(file: String) -> Self {
        let rows: Vec<Row> = file
            .lines()
            .map(|l| {
                let chars = l.to_string();
                Row { chars }
            })
            .collect();

        Document { rows }
    }
}

pub struct Row {
    pub chars: String,
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        if start > end {
            return String::from("");
        }
        match self
            .chars
            .get(cmp::max(0, start)..cmp::min(self.chars.len(), end))
        {
            Some(c) => c
                .split("")
                .map(|c| {
                    if c == &'\t'.to_string() {
                        " ".repeat(TAB_STOP)
                    } else {
                        c.to_string()
                    }
                })
                .collect(),
            None => String::from(""),
        }
    }
}

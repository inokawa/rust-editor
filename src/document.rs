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
    chars: String,
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        if start > end {
            return String::from("");
        }
        self.chars
            .get(cmp::max(0, start)..cmp::min(self.chars.len(), end))
            .map(|c| {
                c.split("")
                    .map(|c| {
                        if c == &'\t'.to_string() {
                            " ".repeat(TAB_STOP)
                        } else {
                            c.to_string()
                        }
                    })
                    .collect()
            })
            .unwrap_or(String::from(""))
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }
}

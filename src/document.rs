use std::cmp;

const TAB_STOP: usize = 4;

pub struct Document {
    pub filename: Option<String>,
    pub rows: Vec<Row>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            filename: None,
            rows: vec![],
        }
    }

    pub fn open(filename: String, file: String) -> Self {
        let rows: Vec<Row> = file
            .lines()
            .map(|l| Row {
                string: l.to_string(),
            })
            .collect();

        Document {
            filename: Some(filename),
            rows,
        }
    }
}

pub struct Row {
    string: String,
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        if start > end {
            return String::from("");
        }
        self.string
            .get(cmp::max(0, start)..cmp::min(self.string.len(), end))
            .map(|c| {
                c.chars()
                    .map(|c| {
                        if c == '\t' {
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
        self.string.chars().fold(0, |acc, c| {
            if c == '\t' {
                acc + 1 * TAB_STOP
            } else {
                acc + 1
            }
        })
    }
}

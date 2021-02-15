use super::editor::Position;
use std::cmp;

const TAB_STOP: usize = 4;

pub struct Document {
    pub filename: Option<String>,
    rows: Vec<Row>,
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

    pub fn row(&self, y: usize) -> Option<&Row> {
        self.rows.get(y)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn insert(&mut self, c: char, at: &Position) {
        if at.y == self.len() {
            let mut row = Row::new();
            row.insert(c, 0);
            self.rows.push(row);
        } else if at.y < self.len() {
            if let Some(row) = self.rows.get_mut(at.y) {
                row.insert(c, at.x);
            }
        }
    }

    pub fn delete(&mut self, at: &Position) {
        let len = self.len();
        if at.y >= len {
            return;
        }

        let row_len = match self.rows.get(at.y) {
            Some(row) => row.len(),
            None => return,
        };
        if at.x == row_len && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            if let Some(row) = self.rows.get_mut(at.y) {
                row.append(&next_row);
            }
        } else {
            if let Some(row) = self.rows.get_mut(at.y) {
                row.delete(at.x);
            }
        }
    }
}

pub struct Row {
    string: String,
}

impl Row {
    pub fn new() -> Self {
        Row {
            string: String::from(""),
        }
    }

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

    fn insert(&mut self, c: char, at: usize) {
        if at >= self.len() {
            self.string.push(c);
        } else {
            self.string = self
                .string
                .chars()
                .enumerate()
                .map(|(i, ch)| if i == at { c } else { ch })
                .collect();
        }
    }

    fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }
        self.string = self
            .string
            .chars()
            .enumerate()
            .filter(|(i, _)| i != &at)
            .map(|(_, c)| c)
            .collect();
    }

    fn append(&mut self, new: &Self) {
        self.string.push_str(&new.string);
    }
}

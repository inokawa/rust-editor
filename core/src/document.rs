use super::editor::Position;
use std::cmp;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

const MAX_UNDO_LENGTH: usize = 1000;
const TAB_STOP: usize = 4;

#[derive(Clone)]
enum Action {
    Insert { pos: Position, c: char },
    Delete { pos: Position, c: char },
    InsertRow { y: usize, row: Row },
    DeleteRow { y: usize, row: Row },
    // TODO handle split by enter
    // TODO keep cursor position
}

pub struct Document {
    pub filename: Option<String>,
    rows: Vec<Row>,
    dirty: usize,
    history_index: usize,
    histories: Vec<Action>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            filename: None,
            rows: Vec::new(),
            dirty: 0,
            history_index: 0,
            histories: Vec::new(),
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
            dirty: 0,
            history_index: 0,
            histories: Vec::new(),
        }
    }

    pub fn contents(&self) -> Vec<String> {
        self.rows.iter().map(|r| r.string.clone()).collect()
    }

    pub fn row(&self, y: usize) -> Option<&Row> {
        self.rows.get(y)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty > 0
    }

    pub fn reset_dirty(&mut self) {
        self.dirty = 0;
    }

    fn edit(&mut self) {
        self.dirty += 1;
        self.histories = self.histories[..(self.histories.len() - self.history_index)].to_vec();
        self.history_index = 0;
    }

    fn push_history(&mut self, action: Action) {
        self.histories.push(action);
        let len = self.histories.len();
        if len > MAX_UNDO_LENGTH {
            self.histories = self.histories[len - MAX_UNDO_LENGTH..].to_vec();
        }
    }

    pub fn insert_newline(&mut self, at: &Position) {
        if at.y > self.len() {
            return;
        }
        let r;
        if at.y == self.len() || at.y + 1 == self.len() {
            r = Row::new();
            self.rows.push(r.clone());
        } else if let Some(row) = self.rows.get_mut(at.y) {
            r = row.split(at.x);
            self.rows.insert(at.y + 1, r.clone());
        } else {
            return;
        }
        self.edit();
        self.push_history(Action::InsertRow {
            y: at.y + 1,
            row: r,
        });
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
        self.edit();
        self.push_history(Action::Insert {
            pos: Position { x: at.x, y: at.y },
            c,
        });
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
                self.push_history(Action::DeleteRow {
                    y: at.y + 1,
                    row: next_row,
                });
            }
        } else {
            if let Some(row) = self.rows.get_mut(at.y) {
                if let Some(deleted) = row.delete(at.x) {
                    self.push_history(Action::Delete {
                        pos: Position { x: at.x, y: at.y },
                        c: deleted,
                    });
                }
            }
        }
        self.edit();
    }

    pub fn undo(&mut self) {
        let index = self.histories.len() - self.history_index;
        if index == 0 {
            return;
        }
        match self.histories.get(index - 1) {
            Some(action) => {
                match action {
                    Action::Insert { pos, c } => {
                        if let Some(row) = self.rows.get_mut(pos.y) {
                            if let Some(_) = row.delete(pos.x) {}
                        }
                    }
                    Action::Delete { pos, c } => {
                        if let Some(row) = self.rows.get_mut(pos.y) {
                            row.insert(c.clone(), pos.x);
                        }
                    }
                    Action::InsertRow { y, row } => {
                        self.rows.remove(y.clone());
                    }
                    Action::DeleteRow { y, row } => {
                        self.rows.insert(y.clone(), row.clone());
                    }
                }
                self.history_index += 1;
            }
            _ => {}
        }
    }

    pub fn redo(&mut self) {
        let index = self.histories.len() - self.history_index;
        if index == self.histories.len() {
            return;
        }
        match self.histories.get(index) {
            Some(action) => {
                match action {
                    Action::Insert { pos, c } => {
                        if let Some(row) = self.rows.get_mut(pos.y) {
                            row.insert(c.clone(), pos.x);
                        }
                    }
                    Action::Delete { pos, c } => {
                        if let Some(row) = self.rows.get_mut(pos.y) {
                            if let Some(_) = row.delete(pos.x) {}
                        }
                    }
                    Action::InsertRow { y, row } => {
                        self.rows.insert(y.clone(), row.clone());
                    }
                    Action::DeleteRow { y, row } => {
                        self.rows.remove(y.clone());
                    }
                }
                self.history_index -= 1;
            }
            _ => {}
        }
    }

    pub fn find(&self, query: &str) -> Option<Position> {
        for (y, row) in self.rows.iter().enumerate() {
            if let Some(x) = row.find(query) {
                return Some(Position { x, y });
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct Row {
    string: String,
}

impl Row {
    pub fn new() -> Self {
        Row {
            string: String::new(),
        }
    }

    pub fn render(&self, start: usize, end: usize) -> String {
        if start > end {
            return String::new();
        }
        let start = cmp::max(0, start);
        let end = cmp::min(self.string.len(), end);
        self.string
            .get(start..end)
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
            .unwrap_or(String::new())
    }

    pub fn get_width(&self, start: usize, end: usize) -> usize {
        let start = cmp::max(0, start);
        let end = cmp::min(self.string.graphemes(true).count(), end);
        self.string
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .fold(0, |acc, s| {
                if s == "\t" {
                    acc + 1 * TAB_STOP
                } else {
                    acc + UnicodeWidthStr::width(s)
                }
            })
    }

    pub fn len(&self) -> usize {
        self.string.graphemes(true).fold(0, |acc, c| {
            if c == "\t" {
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
            let mut first: String = self.string.graphemes(true).take(at).collect();
            let rest: String = self.string.graphemes(true).skip(at).collect();
            first.push(c);
            self.string = first + &rest;
        }
    }

    fn delete(&mut self, at: usize) -> Option<char> {
        if at >= self.len() {
            return None;
        }
        let first: String = self.string.graphemes(true).take(at).collect();
        let mut rest = self.string.graphemes(true).skip(at);
        let deleted = rest.next().and_then(|s| s.chars().nth(0));
        let rest: String = rest.collect();
        self.string = first + &rest;
        deleted
    }

    fn append(&mut self, new: &Self) {
        self.string.push_str(&new.string);
    }

    fn split(&mut self, at: usize) -> Self {
        let first = self.string.graphemes(true).take(at).collect();
        let rest = self.string.graphemes(true).skip(at).collect();
        self.string = first;
        Row { string: rest }
    }

    fn find(&self, query: &str) -> Option<usize> {
        let matching_byte_index = self.string.find(query);
        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in self.string.grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    return Some(grapheme_index);
                }
            }
        }
        None
    }
}

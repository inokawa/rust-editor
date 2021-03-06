use super::{
    ansi_escape::*,
    editor::{Position, SearchDirection},
    languages::{Flag, Language},
    tokenizer::*,
};
use std::cmp;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

const MAX_UNDO_LENGTH: usize = 1000;
const TAB_STOP: usize = 4;

#[derive(Clone)]
enum History {
    Insert { pos: Position, c: char },
    Delete { pos: Position, c: char },
    InsertRow { y: usize, row: Row },
    DeleteRow { y: usize, row: Row },
    SplitRow { x: usize, y: usize, row: Row },
    JoinRow { x: usize, y: usize, row: Row },
    // TODO keep cursor position
}

pub struct Document {
    filename: Option<String>,
    rows: Vec<Row>,
    dirty: usize,
    history_index: usize,
    histories: Vec<History>,
    pub language: Language,
}

impl Document {
    pub fn new() -> Self {
        Document {
            filename: None,
            rows: Vec::new(),
            dirty: 0,
            history_index: 0,
            histories: Vec::new(),
            language: Language::Unknown,
        }
    }

    pub fn open(filename: String, file: String) -> Self {
        let rows: Vec<Row> = file
            .lines()
            .map(|l| Row {
                string: l.to_string(),
                highlight: Vec::new(),
            })
            .collect();

        let language = Language::detect(&filename);
        Document {
            filename: Some(filename),
            rows,
            dirty: 0,
            history_index: 0,
            histories: Vec::new(),
            language,
        }
    }

    pub fn get_filename(&self) -> Option<String> {
        self.filename.clone()
    }

    pub fn set_filename(&mut self, filename: Option<String>) {
        self.filename = filename.clone();
        self.language = Language::detect(&filename.unwrap_or(String::new()));
    }

    pub fn contents(&self) -> Vec<String> {
        self.rows.iter().map(|r| r.string.clone()).collect()
    }

    pub fn row(&self, y: usize) -> Option<&Row> {
        self.rows.get(y)
    }

    pub fn render_row(&mut self, y: usize, start: usize, end: usize) -> String {
        if let Some(row) = self.rows.get_mut(y) {
            row.render(start, end)
        } else {
            String::new()
        }
    }

    pub fn update_highlights(&mut self) {
        let flags = self.language.flags();
        for row in &mut self.rows {
            row.update_highlight(flags);
        }
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

    fn edited(&mut self, action: History) {
        self.dirty += 1;
        self.histories = self.histories[..(self.histories.len() - self.history_index)].to_vec();
        self.history_index = 0;

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
        if let Some(row) = self.rows.get_mut(at.y) {
            let r = row.split(at.x);
            self.rows.insert(at.y + 1, r.clone());
            self.edited(History::SplitRow {
                x: at.x,
                y: at.y,
                row: r,
            });
        } else {
            let r = Row::new();
            self.rows.push(r.clone());
            self.edited(History::InsertRow {
                y: at.y + 1,
                row: r,
            });
        }
    }

    pub fn insert(&mut self, c: char, at: &Position) {
        if at.y == self.len() {
            let mut row = Row::new();
            row.insert(c, 0);
            self.rows.push(row);
            self.edited(History::Insert {
                pos: Position { x: at.x, y: at.y },
                c,
            });
        } else if at.y < self.len() {
            if let Some(row) = self.rows.get_mut(at.y) {
                row.insert(c, at.x);
                self.edited(History::Insert {
                    pos: Position { x: at.x, y: at.y },
                    c,
                });
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
                self.edited(History::JoinRow {
                    x: at.x,
                    y: at.y,
                    row: next_row,
                });
            }
        } else {
            if let Some(row) = self.rows.get_mut(at.y) {
                if let Some(deleted) = row.delete(at.x) {
                    self.edited(History::Delete {
                        pos: Position { x: at.x, y: at.y },
                        c: deleted,
                    });
                }
            }
        }
    }

    pub fn undo(&mut self) {
        let index = self.histories.len() - self.history_index;
        if index == 0 {
            return;
        }
        match self.histories.get(index - 1) {
            Some(action) => {
                match action {
                    History::Insert { pos, c } => {
                        if let Some(row) = self.rows.get_mut(pos.y) {
                            if let Some(_) = row.delete(pos.x) {}
                        }
                    }
                    History::Delete { pos, c } => {
                        if let Some(row) = self.rows.get_mut(pos.y) {
                            row.insert(c.clone(), pos.x);
                        }
                    }
                    History::InsertRow { y, row } => {
                        self.rows.remove(y.clone());
                    }
                    History::DeleteRow { y, row } => {
                        self.rows.insert(y.clone(), row.clone());
                    }
                    History::SplitRow { x, y, row } => {
                        if let Some(org_row) = self.rows.get_mut(y.clone()) {
                            org_row.append(row);
                            self.rows.remove(y + 1);
                        }
                    }
                    History::JoinRow { x, y, row } => {
                        if let Some(row) = self.rows.get_mut(y.clone()) {
                            let rest = row.split(x.clone());
                            self.rows.insert(y + 1, rest);
                        }
                    }
                }
                self.history_index += 1;
            }
            None => {}
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
                    History::Insert { pos, c } => {
                        if let Some(row) = self.rows.get_mut(pos.y) {
                            row.insert(c.clone(), pos.x);
                        }
                    }
                    History::Delete { pos, c } => {
                        if let Some(row) = self.rows.get_mut(pos.y) {
                            if let Some(_) = row.delete(pos.x) {}
                        }
                    }
                    History::InsertRow { y, row } => {
                        self.rows.insert(y.clone(), row.clone());
                    }
                    History::DeleteRow { y, row } => {
                        self.rows.remove(y.clone());
                    }
                    History::SplitRow { x, y, row } => {
                        if let Some(row) = self.rows.get_mut(y.clone()) {
                            let rest = row.split(x.clone());
                            self.rows.insert(y + 1, rest);
                        }
                    }
                    History::JoinRow { x, y, row } => {
                        if let Some(org_row) = self.rows.get_mut(y.clone()) {
                            org_row.append(row);
                            self.rows.remove(y + 1);
                        }
                    }
                }
                self.history_index -= 1;
            }
            None => {}
        }
    }

    pub fn find(
        &self,
        query: &str,
        at: &Position,
        direction: &SearchDirection,
    ) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }

        let (start, end) = match direction {
            SearchDirection::Forward => (at.y, self.rows.len()),
            SearchDirection::Backward => (0, at.y.saturating_add(1)),
        };
        let mut position = Position { x: at.x, y: at.y };
        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(&query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }
                match direction {
                    SearchDirection::Forward => {
                        position.y = position.y.saturating_add(1);
                        position.x = 0;
                    }
                    SearchDirection::Backward => {
                        position.y = position.y.saturating_sub(1);
                        position.x = self.rows[position.y].len();
                    }
                };
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct Row {
    string: String,
    highlight: Vec<Token>,
}

impl Row {
    pub fn new() -> Self {
        Row {
            string: String::new(),
            highlight: Vec::new(),
        }
    }

    pub fn render(&self, start: usize, end: usize) -> String {
        if start > end {
            return String::new();
        }
        let start = cmp::max(0, start);
        let end = cmp::min(self.string.len(), end);
        let mut highlight = &Highlight::None;
        let mut string = self
            .string
            .get(start..end)
            .map(|s| {
                s.graphemes(true)
                    .enumerate()
                    .map(|(i, c)| match c {
                        "\t" => " ".repeat(TAB_STOP),
                        _ => {
                            let mut hl = String::new();
                            let h = self
                                .highlight
                                .iter()
                                .find(|h| h.index == start + i)
                                .map(|h| &h.highlight)
                                .unwrap_or(&Highlight::None);
                            if highlight != h {
                                highlight = h;
                                hl.push_str(highlight.color());
                            }
                            format!("{}{}", hl, c)
                        }
                    })
                    .collect()
            })
            .unwrap_or(String::new());
        string.push_str(&COLOR_DEFAULT);
        string
    }

    pub fn update_highlight(&mut self, flags: &[&Flag]) {
        let mut highlight = Vec::new();
        let mut is_prev_sep = true;
        let mut prev_highlight = Highlight::None;
        self.string.graphemes(true).enumerate().for_each(|(i, s)| {
            if flags.contains(&&Flag::Number) {
                if (is_digit(s) && (is_prev_sep || prev_highlight == Highlight::Number))
                    || s == "." && prev_highlight == Highlight::Number
                {
                    highlight.push(Token {
                        index: i,
                        highlight: Highlight::Number,
                    });
                    prev_highlight = Highlight::Number;
                    is_prev_sep = false;
                    return;
                }
            }

            prev_highlight = Highlight::None;
            is_prev_sep = is_separator(s);
        });
        self.highlight = highlight;
    }

    pub fn calc_width(&self, start: usize, end: usize) -> usize {
        let start = cmp::max(0, start);
        let end = cmp::min(self.string.graphemes(true).count(), end);
        self.string
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .fold(0, |acc, s| acc + str_to_width(s))
    }

    pub fn calc_x(&self, prev_x: usize, prev_row: &Row) -> usize {
        if prev_x == 0 {
            return 0;
        }
        let target_width = prev_row.calc_width(0, prev_x);
        let mut x = 0;
        let mut w = 0;
        for s in self.string.graphemes(true) {
            w += str_to_width(s);
            x += 1;
            if w >= target_width {
                if w != target_width {
                    x -= 1;
                }
                break;
            }
        }
        x
    }

    pub fn len(&self) -> usize {
        self.string.graphemes(true).fold(0, |acc, s| match s {
            "\t" => acc + 1 * TAB_STOP,
            _ => acc + 1,
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
        Row {
            string: rest,
            highlight: Vec::new(),
        }
    }

    fn find(&self, query: &str, at: usize, direction: &SearchDirection) -> Option<usize> {
        if at > self.len() {
            return None;
        }
        let (start, end) = match direction {
            SearchDirection::Forward => (at, self.len()),
            SearchDirection::Backward => (0, at),
        };
        let substring: String = self
            .string
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();
        let matching_byte_index = match direction {
            SearchDirection::Forward => substring.find(query),
            SearchDirection::Backward => substring.rfind(query),
        };
        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in substring.grapheme_indices(true).enumerate() {
                if matching_byte_index == byte_index {
                    return Some(start + grapheme_index);
                }
            }
        }
        None
    }
}

fn str_to_width(s: &str) -> usize {
    match s {
        "\t" => 1 * TAB_STOP,
        _ => UnicodeWidthStr::width(s),
    }
}

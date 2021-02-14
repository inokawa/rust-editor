pub struct Document {
    pub num_rows: usize,
    pub row: Row,
}

impl Document {
    pub fn new() -> Self {
        Document {
            num_rows: 0,
            row: Row {
                chars: String::from(""),
            },
        }
    }

    pub fn open(file: String) -> Self {
        let mut rows = String::from("");
        for value in file.lines() {
            rows.push_str(value);
        }
        Document {
            num_rows: 1,
            row: Row { chars: rows },
        }
    }
}

pub struct Row {
    pub chars: String,
}

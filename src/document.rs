pub struct Document {
    pub rows: Vec<Row>,
}

impl Document {
    pub fn new() -> Self {
        Document { rows: vec![] }
    }

    pub fn open(file: String) -> Self {
        let mut rows: Vec<Row> = vec![];
        for value in file.lines() {
            rows.push(Row {
                chars: value.to_string(),
            });
        }
        Document { rows }
    }
}

pub struct Row {
    pub chars: String,
}

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
            .map(|l| Row {
                chars: l.to_string(),
            })
            .collect();

        Document { rows }
    }
}

pub struct Row {
    pub chars: String,
}

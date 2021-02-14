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
}

pub struct Row {
    pub chars: String,
}

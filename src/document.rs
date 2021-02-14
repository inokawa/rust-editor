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
                let render: String = chars
                    .split("")
                    .map(|c| {
                        if c == &'\t'.to_string() {
                            " ".repeat(TAB_STOP)
                        } else {
                            c.to_string()
                        }
                    })
                    .collect();
                Row { chars, render }
            })
            .collect();

        Document { rows }
    }
}

pub struct Row {
    pub chars: String,
    pub render: String,
}

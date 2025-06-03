pub struct Matrix {
    data: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        Matrix { data }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no == 0 {
            return None;
        }
        if row_no > self.data.len() {
            return None;
        }
        self.data.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if self.data.is_empty() {
            return None;
        }
        if col_no > self.data[0].len() {
            return None;
        }
        let mut column = Vec::new();
        for row in &self.data {
            if let Some(&value) = row.get(col_no - 1) {
                column.push(value);
            } else {
                column.push(0); // Default fallback if column is missing
            }
        }
        Some(column)
    }
}

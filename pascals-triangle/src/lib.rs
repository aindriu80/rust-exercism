pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = Vec::new();
        if row_count == 0 {
            return PascalsTriangle { rows };
        }
        for i in 0..row_count {
            let mut new_row = Vec::new();
            // Each row has (i + 1) elements
            for j in 0..=i {
                if j == 0 || j == i {
                    // First and last elements are always 1
                    new_row.push(1);
                } else {
                    // Middle elements are sum of two elements from the previous row
                    let prev_row = &rows[(i - 1) as usize];
                    let sum = prev_row[(j - 1) as usize] + prev_row[j as usize];
                    new_row.push(sum);
                }
            }
            rows.push(new_row);
        }

        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

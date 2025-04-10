pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    // Return empty Vec if size is 0
    if size == 0 {
        return Vec::new();
    }

    let size = size as usize;
    let mut matrix = vec![vec![0u32; size]; size];

    // Defining the boundaries
    let mut row_start = 0;
    let mut row_end = size - 1;
    let mut col_start = 0;
    let mut col_end = size - 1;

    let mut current = 1u32;

    while row_start <= row_end && col_start <= col_end {
        // Fill top row
        for col in col_start..=col_end {
            matrix[row_start][col] = current;
            current += 1;
        }
        row_start += 1;

        // Fill  rightmost column
        for row in row_start..=row_end {
            matrix[row][col_end] = current;
            current += 1;
        }
        if col_end > 0 {
            col_end -= 1;
        } else {
            break;
        }
        if row_start <= row_end {
            for col in (col_start..=col_end).rev() {
                matrix[row_end][col] = current;
                current += 1;
            }
            row_end -= 1;
        }

        if col_start <= col_end {
            for row in (row_start..=row_end).rev() {
                matrix[row][col_start] = current;
                current += 1;
            }
            col_start += 1;
        }
    }
    matrix
}

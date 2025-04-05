pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() || input[0].is_empty() {
        return Vec::new(); // Return an empty vector
    }
    let mut result = Vec::new();

    // for each row and column
    for (row_idx, row) in input.iter().enumerate() {
        if row.is_empty() {
            continue;
        }

        for (col_idx, &value) in row.iter().enumerate() {
            let is_row_max = row.iter().all(|&r| r <= value);

            // Checking if this value is the minimum in its column
            let is_col_min = input
                .iter()
                .all(|r| r.get(col_idx).map_or(true, |&c| c >= value));

            // If both conditions are met, this is a saddle point ->
            if is_row_max && is_col_min {
                result.push((row_idx, col_idx))
            }
        }
    }
    result
}

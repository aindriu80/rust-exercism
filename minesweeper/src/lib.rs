pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }
    // Convert &[&str] to Vec<Vec<u8>>
    let mut field: Vec<Vec<u8>> = minefield.iter().map(|&s| s.as_bytes().to_vec()).collect();
    let rows = field.len();
    let cols = field[0].len();

    // Offsets for the 8 possible neighbours
    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    // Iterate through each cell in the field
    for row in 0..rows {
        for col in 0..cols {
            if field[row][col] == b' ' {
                // Only process empty spaces
                let mut mine_count = 0;

                // Check all 8 neighbouring cells
                for (dx, dy) in &directions {
                    let new_row = row as isize + dx;
                    let new_col = col as isize + dy;

                    // Ensure the new position is within bonds
                    if new_row >= 0
                        && new_row < rows as isize
                        && new_col >= 0
                        && new_col < cols as isize
                        && field[new_row as usize][new_col as usize] == b'*'
                    {
                        mine_count += 1;
                    }
                }

                // If there are mines nearby, uypdate the cell with the count
                if mine_count > 0 {
                    field[row][col] = b'0' + mine_count; // Convert count to ASCII
                }
            }
        }
    }

    // Convert Vec<Vec<u8>> back to Vec<String>
    field
        .iter()
        .map(|row| String::from_utf8(row.clone()).unwrap())
        .collect()
}

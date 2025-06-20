pub fn count(lines: &[&str]) -> u32 {
    let rows = lines.len();
    if rows == 0 {
        return 0;
    }
    let cols = lines[0].len();
    if cols == 0 {
        return 0;
    }

    let mut count = 0;

    for row1 in 0..rows {
        for row2 in row1 + 1..rows {
            for col1 in 0..cols {
                for col2 in col1 + 1..cols {
                    if is_rectangle(lines, row1, col1, row2, col2) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn is_rectangle(lines: &[&str], r1: usize, c1: usize, r2: usize, c2: usize) -> bool {
    // Corners must be '+'
    if lines[r1].as_bytes()[c1] != b'+'
        || lines[r1].as_bytes()[c2] != b'+'
        || lines[r2].as_bytes()[c1] != b'+'
        || lines[r2].as_bytes()[c2] != b'+'
    {
        return false;
    }

    // Top and bottom sides must be '-' or '+'
    for c in c1 + 1..c2 {
        if !matches!(lines[r1].as_bytes()[c], b'-' | b'+')
            || !matches!(lines[r2].as_bytes()[c], b'-' | b'+')
        {
            return false;
        }
    }

    // Left and right sides must be '|' or '+'
    for r in r1 + 1..r2 {
        if !matches!(lines[r].as_bytes()[c1], b'|' | b'+')
            || !matches!(lines[r].as_bytes()[c2], b'|' | b'+')
        {
            return false;
        }
    }

    true
}

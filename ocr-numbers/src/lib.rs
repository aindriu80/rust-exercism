use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(lines.len()));
    }

    if let Some(first_line) = lines.first() {
        let expected_width = first_line.len();
        if expected_width % 3 != 0 {
            return Err(Error::InvalidColumnCount(expected_width));
        }

        for line in &lines {
            if line.len() != expected_width {
                return Err(Error::InvalidColumnCount(line.len()));
            }
        }
    }

    let patterns = create_digit_patterns();

    let mut results = Vec::new();

    for chunk_start in (0..lines.len()).step_by(4) {
        let chunk = &lines[chunk_start..chunk_start + 4];
        let line_result = convert_line(chunk, &patterns)?;
        results.push(line_result);
    }
    Ok(results.join(","))
}

fn convert_line(four_rows: &[&str], patterns: &HashMap<String, char>) -> Result<String, Error> {
    let width = four_rows[0].len();
    let num_digits = width / 3;
    let mut result = String::new();

    // Process each 3-column digit
    for digit_index in 0..num_digits {
        let start_col = digit_index * 3;
        let end_col = start_col + 3;

        // Extract the 3x4 patter for this digit
        let mut pattern = String::new();
        for row in four_rows {
            let digit_part = &row[start_col..end_col];
            pattern.push_str(digit_part);
        }

        // Look up the digit
        let digit = patterns.get(&pattern).unwrap_or(&'?');
        result.push(*digit);
    }
    Ok(result)
}

fn create_digit_patterns() -> HashMap<String, char> {
    let mut patterns = HashMap::new();

    patterns.insert(" _ | ||_|   ".to_string(), '0');

    // 1: "   ", "  |", "  |", "   "
    patterns.insert("     |  |   ".to_string(), '1');

    // 2: " _ ", " _|", "|_ ", "   "
    patterns.insert(" _  _||_    ".to_string(), '2');

    // 3: " _ ", " _|", " _|", "   "
    patterns.insert(" _  _| _|   ".to_string(), '3');

    // 4: "   ", "|_|", "  |", "   "
    patterns.insert("   |_|  |   ".to_string(), '4');

    // 5: " _ ", "|_ ", " _|", "   "
    patterns.insert(" _ |_  _|   ".to_string(), '5');

    // 6: " _ ", "|_ ", "|_|", "   "
    patterns.insert(" _ |_ |_|   ".to_string(), '6');

    // 7: " _ ", "  |", "  |", "   "
    patterns.insert(" _   |  |   ".to_string(), '7');

    // 8: " _ ", "|_|", "|_|", "   "
    patterns.insert(" _ |_||_|   ".to_string(), '8');

    // 9: " _ ", "|_|", " _|", "   "
    patterns.insert(" _ |_| _|   ".to_string(), '9');

    patterns
}

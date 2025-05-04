pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let normalized: String = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    // if normalized text is empty, return empty string
    if normalized.is_empty() {
        return String::new();
    }

    let len = normalized.len();
    let c = (len as f64).sqrt().ceil() as usize; // Number of columns 
    let r = (len + c - 1) / c; // Number of rows (ceiling division )

    // Create the ciphertext by reading column by column
    let mut result = String::with_capacity(len + r - 1); // Reserved space for text and spaces 

    for col in 0..c {
        for row in 0..r {
            let index = row * c + col;
            if index < len {
                result.push(normalized.chars().nth(index).unwrap());
            } else {
                result.push(' ') // Padding with spaces
            }
        }

        // Add space betweeen chunks , but not after the last chunk
        if col < c - 1 {
            result.push(' ');
        }
    }
    result
}

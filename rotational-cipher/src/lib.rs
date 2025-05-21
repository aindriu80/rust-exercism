pub fn rotate(input: &str, key: u8) -> String {
    // Ensure key is within 0-25 range (26 positions in alphabet)
    let key = key % 26;

    // Process each character in the input string
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                // Determine the base value (ASCII 'a' or 'A')
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };

                // Convert to 0-25 range, apply rotation, then back to ASCII
                let offset = (c as u8 - base + key) % 26;
                (base + offset) as char
            } else {
                // Non-alphabetic characters remain unchanged
                c
            }
        })
        .collect()
}

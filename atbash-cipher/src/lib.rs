/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    if plain.is_empty() {
        return String::new();
    }

    // Perform the Atbash substitution and filter out punctuation
    let encoded: String = plain
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphabetic() {
                // For lowercase and uppercase, convert to lowercase first
                let c_lower = c.to_ascii_lowercase();
                // Apply Atbash formula: 'a' + 'z' - c
                Some((b'a' + b'z' - c_lower as u8) as char)
            } else if c.is_ascii_digit() {
                // Keep digits
                Some(c)
            } else {
                // Skip other characters like spaces, punctuation, etc.
                None
            }
        })
        .collect();

    // Group encoded text into chunks of 5 characters
    let mut result = String::new();
    for (i, c) in encoded.chars().enumerate() {
        if i > 0 && i % 5 == 0 {
            result.push(' ');
        }
        result.push(c);
    }

    result
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    if cipher.is_empty() {
        return String::new();
    }

    // Remove spaces and apply Atbash decoding (which is the same as encoding)
    cipher
        .chars()
        .filter(|&c| c != ' ') // Remove spaces
        .map(|c| {
            if c.is_ascii_lowercase() {
                // For lowercase: apply Atbash formula
                (b'a' + b'z' - c as u8) as char
            } else if c.is_ascii_uppercase() {
                // For uppercase: convert to lowercase and apply formula
                let c_lower = c.to_ascii_lowercase();
                (b'a' + b'z' - c_lower as u8) as char
            } else if c.is_ascii_digit() {
                // Keep digits as they are
                c
            } else {
                // Skip other characters (shouldn't happen based on encode)
                c
            }
        })
        .collect()
}

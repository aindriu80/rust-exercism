pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 || len > digits.len() {
        return vec![];
    }
    digits
        .as_bytes() // Convert to byte slice
        .windows(len) // Now works, since this is a slice method
        .map(|window| String::from_utf8_lossy(window).to_string()) // Convert bytes back to String
        .collect()
}

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "");

    // Check if the resulting string has exactly 10 characters
    if isbn.len() != 10 {
        return false;
    }

    // Calculate the checksum
    let mut sum = 0;
    for (i, c) in isbn.chars().take(9).enumerate() {
        // Convert character to digit
        let digit = match c.to_digit(10) {
            Some(d) => d,
            None => return false, // Invalid character
        };
        sum += digit * (10 - i as u32);
    }

    // Handle the last character (can be 'X' or a digit)
    let last_char = isbn.chars().nth(9).unwrap();
    let last_digit = if last_char == 'X' {
        10
    } else {
        match last_char.to_digit(10) {
            Some(d) => d,
            None => return false, // Invalid character
        }
    };

    // Add the last digit to the sum
    sum += last_digit;

    // Check if the sum is divisible by 11
    sum % 11 == 0
}

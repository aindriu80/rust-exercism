pub fn is_valid(code: &str) -> bool {
    // Remove all whitespace
    let cleaned_code: String = code.chars().filter(|c| !c.is_whitespace()).collect();

    // Ensure length is greater than 1
    if cleaned_code.len() <= 1 {
        return false;
    }

    // Ensure all characters are digits
    if !cleaned_code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // Reverse the cleaned string to process from right to left
    let cleaned_code = cleaned_code.chars().rev().collect::<String>();

    // Implement the actual Luhn Algorithm
    let mut sum = 0;

    for (i, c) in cleaned_code.chars().enumerate() {
        let mut digit = c.to_digit(10).unwrap() as u32;

        // Double every second digit, starting from the right (index 1, 3, 5,...)
        if i % 2 == 1 {
            digit *= 2;

            // If doubling results in a number greater than 9, subtract 9 (equivalent to summing the digits)
            if digit > 9 {
                digit -= 9;
            }
        }

        sum += digit;
    }

    // Check if the sum is divisible by 10
    sum % 10 == 0
}

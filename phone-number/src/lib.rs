pub fn number(user_number: &str) -> Option<String> {
    let digits: String = user_number.chars().filter(|c| c.is_ascii_digit()).collect();

    match digits.len() {
        10 => {
            if digits.chars().nth(0) == Some('0') || digits.chars().nth(0) == Some('1') {
                return None;
            }
            if digits.chars().nth(3) == Some('0') || digits.chars().nth(3) == Some('1') {
                return None;
            }
            Some(digits)
        }
        11 => {
            if digits.chars().nth(0) != Some('1') {
                return None;
            }
            if digits.chars().nth(1) == Some('0') || digits.chars().nth(1) == Some('1') {
                return None;
            }
            if digits.chars().nth(4) == Some('0') || digits.chars().nth(4) == Some('1') {
                return None;
            }
            Some(digits[1..].to_string())
        }
        _ => None,
    }
}

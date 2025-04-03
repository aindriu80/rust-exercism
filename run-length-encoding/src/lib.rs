pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut count = 1;
    let mut chars = source.chars().peekable();
    let mut current_char = chars.next().unwrap();

    while let Some(c) = chars.next() {
        if c == current_char {
            count += 1;
        } else {
            // If count is greater than 1, add the count to the result
            if count > 1 {
                result.push_str(&count.to_string());
            }
            result.push(current_char);
            current_char = c;
            count = 1;
        }
    }
    if count > 1 {
        result.push_str(&count.to_string());
    }
    result.push(current_char);
    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            let mut count_str = c.to_string();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_digit(10) {
                    count_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            let count: usize = count_str.parse().unwrap_or(0);

            if let Some(letter) = chars.next() {
                for _ in 0..count {
                    result.push(letter)
                }
            }
        } else {
            result.push(c)
        }
    }
    result
}

pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    let mut prev_char = ' '; // Start with space to take first letter
    let delimiters = [' ', '-', '_']; // <- Removed '\'' from this list

    for (i, c) in phrase.chars().enumerate() {
        if c.is_alphabetic() {
            // Take first letter after delimiter or space
            let after_delimiter = prev_char.is_whitespace() || delimiters.contains(&prev_char);
            // Take uppercase in camelCase (but not if it's the first char after delimiter)
            let camel_case_transition = i > 0
                && prev_char.is_lowercase()
                && c.is_uppercase()
                && !delimiters.contains(&phrase.chars().nth(i - 1).unwrap());

            if after_delimiter || camel_case_transition {
                result.push(c);
            }
        }
        prev_char = c;
    }

    result.to_uppercase()
}

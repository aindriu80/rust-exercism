// pub fn reply(message: &str) -> &str {
//     if message.chars().last() == Some('?') {
//         return "Sure.";
//     }
//     if message.chars().any(|c| c.is_ascii_uppercase()) {
//         return "Whoa, chill out!";
//     }
//     if message == ("") {
//         return "Fine. Be that way!";
//     } else {
//         return "Whatever.";
//     }
// }

pub fn reply(message: &str) -> &str {
    // Trim whitespace to handle trailing/leading spaces
    let trimmed = message.trim();

    // Check for silence first (empty or only whitespace)
    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }

    // Check if it's a question (ends with '?')
    let is_question = trimmed.ends_with('?');
    // Check if it's yelling (all alphabetic chars are uppercase and contains at least one letter)
    let has_letters = trimmed.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_letters
        && trimmed
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase());

    // Handle the cases based on the conditions
    if is_yelling && is_question {
        "Calm down, I know what I'm doing!"
    } else if is_question {
        "Sure."
    } else if is_yelling {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}

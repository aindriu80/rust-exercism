/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    if sentence.is_empty() {
        return false;
    }
    let mut alphabet = std::collections::HashSet::new();

    // Iterate through the sentence, converting to lowercase
    for ch in sentence.to_lowercase().chars() {
        // Only add letters a-z
        if ch.is_ascii_alphabetic() {
            alphabet.insert(ch);
        }
    }
    alphabet.len() == 26
    // false
}

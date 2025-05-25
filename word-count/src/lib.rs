use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut word_counts: HashMap<String, u32> = HashMap::new();

    // Split the string into words, handling punctuation
    let words = words.split(|c: char| {
        c.is_whitespace() || (c.is_ascii_punctuation() && c != '\'' && c != '"' && c != '\'')
    });

    for word in words {
        let mut cleaned_word = word.trim();

        // Remove quotes from beginning and end
        while cleaned_word.starts_with('\'') || cleaned_word.starts_with('"') {
            cleaned_word = &cleaned_word[1..];
        }
        while cleaned_word.ends_with('\'') || cleaned_word.ends_with('"') {
            cleaned_word = &cleaned_word[..cleaned_word.len() - 1];
        }

        let cleaned_word = cleaned_word.to_lowercase();
        if !cleaned_word.is_empty() {
            *word_counts.entry(cleaned_word).or_insert(0) += 1;
        }
    }

    word_counts
}

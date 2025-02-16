use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let normalized_word = normalize(word);
    let mut valid_anagrams = HashSet::new();

    for &candidate in possible_anagrams {
        if candidate.to_lowercase() != word.to_lowercase()
            && is_anagram(&normalized_word, candidate)
        {
            valid_anagrams.insert(candidate);
        }
    }

    valid_anagrams
}
fn normalize(word: &str) -> String {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

fn is_anagram(target_normalized: &str, candidate: &str) -> bool {
    normalize(candidate) == target_normalized
}

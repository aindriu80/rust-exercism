pub fn translate(input: &str) -> String {
    // Split the input string into words
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = word.chars().collect();
    let first_char = chars[0];

    // Rule 1: Word begins with a vowel or "xr" or "yt"
    if is_vowel(first_char)
        || (word.len() >= 2 && (word.starts_with("xr") || word.starts_with("yt")))
    {
        return format!("{}ay", word);
    }

    // Rule 3: Word starts with consonants followed by "qu"
    if let Some(qu_pos) = find_substring_position(word, "qu") {
        if all_consonants_before(&chars, qu_pos) {
            let (prefix, remaining) = word.split_at(qu_pos + 2);
            return format!("{}{}ay", remaining, prefix);
        }
    }

    // Rule 4: Word starts with consonants followed by 'y'
    if let Some(y_pos) = chars.iter().position(|&c| c == 'y') {
        // Make sure 'y' is not the first letter (would be handled by Rule 1 if it were)
        if y_pos > 0 && all_consonants_before(&chars, y_pos) {
            let (prefix, remaining) = word.split_at(y_pos);
            return format!("{}{}ay", remaining, prefix);
        }
    }

    // Rule 2: Word begins with one or more consonants
    if let Some(first_vowel_pos) = chars.iter().position(|&c| is_vowel(c)) {
        let (prefix, remaining) = word.split_at(first_vowel_pos);
        return format!("{}{}ay", remaining, prefix);
    }

    // If no vowels at all, just add "ay"
    format!("{}ay", word)
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn all_consonants_before(chars: &[char], pos: usize) -> bool {
    chars[0..pos].iter().all(|&c| !is_vowel(c))
}

fn find_substring_position(s: &str, substring: &str) -> Option<usize> {
    s.find(substring)
}

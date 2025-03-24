use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut letter_counts: HashMap<char, u32> = HashMap::new();

    for c in candidate
        .chars()
        .filter(|&c| c != ' ' && c != '-')
        .map(|c| c.to_ascii_lowercase())
    {
        *letter_counts.entry(c).or_insert(0) += 1;
    }

    letter_counts.values().all(|&count| count == 1)
}

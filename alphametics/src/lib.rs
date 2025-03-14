use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let parts: Vec<&str> = input.split(" == ").collect();
    if parts.len() != 2 {
        return None;
    }
    let addends: Vec<&str> = parts[0].split(" + ").collect();
    let result = parts[1];

    // Collect unique letters
    let letters: Vec<char> = addends
        .iter()
        .chain(std::iter::once(&result))
        .flat_map(|s| s.chars())
        .collect::<HashSet<char>>()
        .into_iter()
        .collect();

    // Track assignments and used digits
    let mut mapping = HashMap::new();
    let mut used = [false; 10];

    // Solve with backtracking
    if solve_recursive(&addends, result, &letters, 0, &mut mapping, &mut used) {
        Some(mapping)
    } else {
        None
    }
}

fn solve_recursive(
    addends: &[&str],
    result: &str,
    letters: &[char],
    index: usize,
    mapping: &mut HashMap<char, u8>,
    used: &mut [bool; 10],
) -> bool {
    // Base case: all letters assigned, check if solution is valid
    if index == letters.len() {
        return is_valid_solution(addends, result, mapping);
    }

    let letter = letters[index];
    // Check if this letter is a leading digit
    let is_leading = addends
        .iter()
        .chain(std::iter::once(&result))
        .any(|word| word.starts_with(letter));
    let start = if is_leading { 1 } else { 0 };

    // Try each digit
    for digit in start..10 {
        if !used[digit] {
            used[digit] = true;
            mapping.insert(letter, digit as u8);

            if solve_recursive(addends, result, letters, index + 1, mapping, used) {
                return true;
            }

            // Backtrack
            used[digit] = false;
            mapping.remove(&letter);
        }
    }
    false
}

fn is_valid_solution(addends: &[&str], result: &str, mapping: &HashMap<char, u8>) -> bool {
    // Convert words to numbers
    let sum: u64 = addends
        .iter()
        .map(|&word| word_to_number(word, mapping))
        .sum();
    let result_num = word_to_number(result, mapping);

    sum == result_num
}

fn word_to_number(word: &str, mapping: &HashMap<char, u8>) -> u64 {
    word.chars().fold(0, |acc, c| acc * 10 + mapping[&c] as u64)
}

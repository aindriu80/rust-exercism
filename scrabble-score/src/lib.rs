/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut score = 0;
    for char in word.chars() {
        if char.is_ascii() {
            match char {
                'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' | 'a' | 'e' | 'i'
                | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => score += 1,
                'D' | 'G' | 'd' | 'g' => score += 2,
                'B' | 'C' | 'M' | 'P' | 'b' | 'c' | 'm' | 'p' => score += 3,
                'F' | 'H' | 'V' | 'W' | 'Y' | 'f' | 'h' | 'v' | 'w' | 'y' => score += 4,
                'K' | 'k' => score += 5,
                'J' | 'X' | 'j' | 'x' => score += 8,
                _ => score += 10,
            }
        }
    }
    score
}

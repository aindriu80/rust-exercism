/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    // Check if both strings have the same length
    if s1.len() != s2.len() {
        None // Return None if lengths are different
    } else {
        let mut distance = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                distance += 1;
            }
        }
        Some(distance) // Wrap the result in Some()
    }
}

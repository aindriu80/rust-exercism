use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // check if nucleotide is valid
    if !matches!(nucleotide, 'A' | 'C' | 'G' | 'T') {
        return Err(nucleotide); // Fail with the invalid nucleotide
    }

    // Check if all characters in dna are valid
    for c in dna.chars() {
        if !matches!(c, 'A' | 'C' | 'G' | 'T') {
            return Err(c); // Fail with the first invalid character in dna
        }
    }

    // Count occurrences of nucleotide in dna
    let count = dna.chars().filter(|&c| c == nucleotide).count();
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    // Check for invalid characters first
    for c in dna.chars() {
        if !matches!(c, 'A' | 'C' | 'G' | 'T') {
            return Err(c); // Return the first invalid character
        }
    }
    // Initialize a HashMap with all nucleotides set to 0
    let mut counts = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);

    // Count each nucleotide in dna
    for c in dna.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    Ok(counts)
}

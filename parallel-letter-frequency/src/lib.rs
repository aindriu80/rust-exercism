use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    input
        .par_iter()
        .map(|text| {
            let mut local_counts: HashMap<char, usize> = HashMap::new();
            for c in text.chars() {
                let lower_c = c.to_lowercase().next().unwrap();
                if lower_c.is_alphabetic() {
                    *local_counts.entry(lower_c).or_insert(0) += 1;
                }
            }
            local_counts
        })
        .reduce(
            || HashMap::new(),
            |mut acc, local_counts| {
                for (char, count) in local_counts {
                    *acc.entry(char).or_insert(0) += count;
                }
                acc
            },
        )
}

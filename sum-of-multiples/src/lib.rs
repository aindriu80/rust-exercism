use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut seen = HashSet::new();

    for &factor in factors.iter().filter(|&&f| f != 0) {
        seen.extend((factor..limit).step_by(factor as usize));
    }

    seen.iter().sum()
}

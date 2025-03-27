use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(value: u64, factors: HashSet<(u64, u64)>) -> Self {
        Palindrome { value, factors }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    // Check if range is valid
    if min > max {
        return None;
    }

    let mut smallest_palindrome: Option<Palindrome> = None;
    let mut largest_palindrome: Option<Palindrome> = None;
    let mut smallest_found = u64::MAX;
    let mut largest_found = 0;

    // Iterate through all possible factor combinations
    for i in min..=max {
        for j in i..=max {
            let product = i * j;

            // Check if product is a palindrome
            if is_palindrome(product) {
                // Update smallest palindrome
                if product < smallest_found {
                    smallest_found = product;
                    smallest_palindrome = Some(Palindrome::new(product, HashSet::from([(i, j)])));
                } else if product == smallest_found {
                    if let Some(mut pal) = smallest_palindrome {
                        pal.factors.insert((i, j));
                        smallest_palindrome = Some(pal);
                    }
                }

                // Update largest palindrome
                if product > largest_found {
                    largest_found = product;
                    largest_palindrome = Some(Palindrome::new(product, HashSet::from([(i, j)])));
                } else if product == largest_found {
                    if let Some(mut pal) = largest_palindrome {
                        pal.factors.insert((i, j));
                        largest_palindrome = Some(pal);
                    }
                }
            }
        }
    }

    // Return both smallest and largest palindromes
    match (smallest_palindrome, largest_palindrome) {
        (Some(smallest), Some(largest)) => Some((smallest, largest)),
        _ => None,
    }
}

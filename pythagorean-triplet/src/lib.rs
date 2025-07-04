use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets: HashSet<[u32; 3]> = HashSet::new();

    for a in 1..sum / 3 {
        for b in (a + 1)..=(sum - a) / 2 {
            let c = sum - a - b;

            if a * a + b * b == c * c {
                let triplet = [a, b, c];
                triplets.insert(triplet);
            }
        }
    }

    triplets
}

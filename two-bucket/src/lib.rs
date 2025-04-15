use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > capacity_1 && goal > capacity_2 {
        return None;
    }
    fn gcd(a: u8, b: u8) -> u8 {
        if b == 0 { a } else { gcd(b, a % b) }
    }
    let g = gcd(capacity_1, capacity_2);
    if g != 0 && goal % g != 0 {
        return None;
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let initial_state = match start_bucket {
        Bucket::One => (capacity_1, 0, 1),
        Bucket::Two => (0, capacity_2, 1),
    };
    queue.push_back(initial_state);
    visited.insert((initial_state.0, initial_state.1));

    while let Some((b1, b2, moves)) = queue.pop_front() {
        if b1 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: b2,
            });
        }
        if b2 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: b1,
            });
        }

        let next_states = [
            (capacity_1, b2),
            (b1, capacity_2),
            (0, b2),
            (b1, 0),
            {
                let pour = (capacity_2 - b2).min(b1);
                (b1 - pour, b2 + pour)
            },
            {
                let pour = (capacity_1 - b1).min(b2);
                (b1 + pour, b2 - pour)
            },
        ];

        for &(next_b1, next_b2) in &next_states {
            let valid = match start_bucket {
                Bucket::One => !(next_b1 == 0 && next_b2 == capacity_2),
                Bucket::Two => !(next_b2 == 0 && next_b1 == capacity_1),
            };

            if valid && visited.insert((next_b1, next_b2)) {
                queue.push_back((next_b1, next_b2, moves + 1))
            }
        }
    }
    None
}

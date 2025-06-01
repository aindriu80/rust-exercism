use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        Category::Yacht => {
            if dice[0] == dice[1] && dice[1] == dice[2] && dice[2] == dice[3] && dice[3] == dice[4]
            {
                50
            } else {
                0
            }
        }

        Category::Choice => {
            let choice = dice[0] + dice[1] + dice[2] + dice[3] + dice[4];
            choice
        }

        Category::BigStraight => {
            let mut seen = HashSet::new();
            for &die in &dice {
                seen.insert(die);
            }
            if seen.contains(&2)
                && seen.contains(&3)
                && seen.contains(&4)
                && seen.contains(&5)
                && seen.contains(&6)
            {
                30
            } else {
                0
            }
        }

        Category::LittleStraight => {
            let mut seen = HashSet::new();
            for &die in &dice {
                seen.insert(die);
            }
            if seen.contains(&1)
                && seen.contains(&2)
                && seen.contains(&3)
                && seen.contains(&4)
                && seen.contains(&5)
            {
                30
            } else {
                0
            }
        }

        Category::FourOfAKind => {
            let mut counts: HashMap<u8, u8> = HashMap::new();
            for &die in &dice {
                // Build the frequency map for all dice first
                *counts.entry(die).or_insert(0) += 1;
            }

            // Now, after all dice have been processed, check the counts
            let mut sum_of_four = 0;
            let mut found_four = false;

            // for &count in counts.values() {
            for (&die, &count) in &counts {
                if count >= 4 {
                    sum_of_four += &die * 4;
                    found_four = true;
                }
            }

            if found_four { sum_of_four } else { 0 }
        }
        Category::FullHouse => {
            let mut counts: HashMap<u8, u8> = HashMap::new();
            for &die in &dice {
                *counts.entry(die).or_insert(0) += 1;
            }

            let mut has_pair = false;
            let mut has_three = false;

            for &count in counts.values() {
                if count == 3 {
                    has_three = true;
                } else if count == 3 {
                    has_three = true;
                } else if count == 2 {
                    has_pair = true;
                }
            }

            if has_pair && has_three {
                let mut sum: u8 = 0;
                for &die in &dice {
                    sum += die;
                }
                sum
            } else {
                0
            }
        }

        Category::Sixes => {
            let mut counts: HashMap<u8, u8> = HashMap::new();
            for &die in &dice {
                // Build the frequency map for all dice first
                *counts.entry(die).or_insert(0) += 1;
            }

            // Get the count for '6' specifically.
            // .get(&6) returns an Option<&u8>.
            // .unwrap_or(&0) safely provides 0 if '6' is not found in the map.
            let num_sixes = *counts.get(&6).unwrap_or(&0);

            // Calculate the score: 6 multiplied by the number of sixes
            (6 * num_sixes) as u8
        }

        Category::Fives => {
            let mut counts: HashMap<u8, u8> = HashMap::new();
            for &die in &dice {
                *counts.entry(die).or_insert(0) += 1;
            }

            let num_sixes = *counts.get(&5).unwrap_or(&0);

            (5 * num_sixes) as u8
        }

        Category::Fours => {
            let mut counts: HashMap<u8, u8> = HashMap::new();
            for &die in &dice {
                *counts.entry(die).or_insert(0) += 1;
            }

            let num_sixes = *counts.get(&4).unwrap_or(&0);

            (4 * num_sixes) as u8
        }

        Category::Threes => {
            let mut counts: HashMap<u8, u8> = HashMap::new();
            for &die in &dice {
                *counts.entry(die).or_insert(0) += 1;
            }

            let num_sixes = *counts.get(&3).unwrap_or(&0);

            (3 * num_sixes) as u8
        }

        Category::Twos => {
            let mut counts: HashMap<u8, u8> = HashMap::new();
            for &die in &dice {
                *counts.entry(die).or_insert(0) += 1;
            }

            let num_sixes = *counts.get(&2).unwrap_or(&0);

            (2 * num_sixes) as u8
        }

        Category::Ones => {
            let mut counts: HashMap<u8, u8> = HashMap::new();
            for &die in &dice {
                *counts.entry(die).or_insert(0) += 1;
            }

            let num_sixes = *counts.get(&1).unwrap_or(&0);

            (1 * num_sixes) as u8
        }
    }
}

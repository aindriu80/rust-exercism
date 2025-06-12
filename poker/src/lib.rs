use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    rank: u8,   // 2=2, 3=3, ..., 10=10, 11=J, 12=Q, 13=K, 14=A
    suit: char, // 'H', 'D', 'C', 'S'
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard(Vec<u8>),         // [highest, second, third, fourth, fifth]
    OnePair(u8, Vec<u8>),      // (pair_rank, [kickers])
    TwoPair(u8, u8, u8),       // (higher_pair, lower_pair, kicker)
    ThreeOfAKind(u8, Vec<u8>), // (trips_rank, [kickers])
    Straight(u8),              // (highest_card)
    Flush(Vec<u8>),            // [highest, second, third, fourth, fifth]
    FullHouse(u8, u8),         // (trips_rank, pair_rank)
    FourOfAKind(u8, u8),       // (quads_rank, kicker)
    StraightFlush(u8),         // (highest_card)
    FiveOfAKind(u8),           // (rank) - only possible with wild cards
}
impl Card {
    fn from_str(s: &str) -> Result<Card, &'static str> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() < 2 {
            return Err("Invalid card format");
        }
        let suit = chars[chars.len() - 1];
        let rank_str: String = chars[..chars.len() - 1].iter().collect();
        let rank = match rank_str.as_str() {
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "10" => 10,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => return Err("Invalid rank"),
        };
        match suit {
            'H' | 'D' | 'C' | 'S' => Ok(Card { rank, suit }),
            _ => Err("Invalid suit"),
        }
    }
}
fn parse_hand(hand: &str) -> Result<Vec<Card>, &'static str> {
    hand.split_whitespace().map(Card::from_str).collect()
}
fn evaluate_hand(cards: &[Card]) -> HandRank {
    let mut rank_counts: HashMap<u8, usize> = HashMap::new();
    let mut suit_counts: HashMap<char, usize> = HashMap::new();
    // Count ranks and suits
    for card in cards {
        *rank_counts.entry(card.rank).or_insert(0) += 1;
        *suit_counts.entry(card.suit).or_insert(0) += 1;
    }
    // Get sorted ranks for easier processing
    let mut sorted_ranks: Vec<u8> = cards.iter().map(|c| c.rank).collect();
    sorted_ranks.sort_by(|a, b| b.cmp(a)); // Sort descending
    // Check for flush
    let is_flush = suit_counts.len() == 1;
    // Check for straight (including A-2-3-4-5 low straight)
    let is_straight = is_consecutive(&sorted_ranks) || is_low_ace_straight(&sorted_ranks);
    let straight_high = if is_low_ace_straight(&sorted_ranks) {
        5
    } else {
        sorted_ranks[0]
    };
    // Group by count frequency
    let mut count_groups: HashMap<usize, Vec<u8>> = HashMap::new();
    for (rank, count) in &rank_counts {
        count_groups
            .entry(*count)
            .or_insert_with(Vec::new)
            .push(*rank);
    }
    // Sort each group by rank (descending)
    for group in count_groups.values_mut() {
        group.sort_by(|a, b| b.cmp(a));
    }
    // Determine hand ranking
    match (is_straight, is_flush) {
        (true, true) => {
            if straight_high == 14 && sorted_ranks.contains(&10) {
                // Royal flush is just a straight flush A-high
                HandRank::StraightFlush(14)
            } else {
                HandRank::StraightFlush(straight_high)
            }
        }
        (false, false) => {
            if let Some(quads) = count_groups.get(&4) {
                let kicker = count_groups.get(&1).unwrap()[0];
                HandRank::FourOfAKind(quads[0], kicker)
            } else if let (Some(trips), Some(pair)) = (count_groups.get(&3), count_groups.get(&2)) {
                HandRank::FullHouse(trips[0], pair[0])
            } else if let Some(trips) = count_groups.get(&3) {
                let mut kickers = count_groups.get(&1).unwrap().clone();
                kickers.sort_by(|a, b| b.cmp(a));
                HandRank::ThreeOfAKind(trips[0], kickers)
            } else if let Some(pairs) = count_groups.get(&2) {
                if pairs.len() == 2 {
                    let mut sorted_pairs = pairs.clone();
                    sorted_pairs.sort_by(|a, b| b.cmp(a));
                    let kicker = count_groups.get(&1).unwrap()[0];
                    HandRank::TwoPair(sorted_pairs[0], sorted_pairs[1], kicker)
                } else {
                    let mut kickers = count_groups.get(&1).unwrap().clone();
                    kickers.sort_by(|a, b| b.cmp(a));
                    HandRank::OnePair(pairs[0], kickers)
                }
            } else {
                HandRank::HighCard(sorted_ranks)
            }
        }
        (true, false) => HandRank::Straight(straight_high),
        (false, true) => HandRank::Flush(sorted_ranks),
    }
}
fn is_consecutive(ranks: &[u8]) -> bool {
    for i in 1..ranks.len() {
        if ranks[i - 1] - ranks[i] != 1 {
            return false;
        }
    }
    true
}
fn is_low_ace_straight(ranks: &[u8]) -> bool {
    // Check for A-2-3-4-5 (wheel)
    let mut sorted = ranks.to_vec();
    sorted.sort();
    sorted == vec![2, 3, 4, 5, 14]
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.is_empty() {
        return vec![];
    }
    // Parse and evaluate all hands
    let mut hand_rankings: Vec<(HandRank, &'a str)> = Vec::new();
    for &hand_str in hands {
        if let Ok(cards) = parse_hand(hand_str) {
            let rank = evaluate_hand(&cards);
            hand_rankings.push((rank, hand_str));
        }
    }
    // Sort by hand ranking (best first)
    hand_rankings.sort_by(|a, b| b.0.cmp(&a.0));
    // Find all hands with the best ranking
    if let Some((best_rank, _)) = hand_rankings.first() {
        hand_rankings
            .iter()
            .take_while(|(rank, _)| rank == best_rank)
            .map(|(_, hand_str)| *hand_str)
            .collect()
    } else {
        vec![]
    }
}

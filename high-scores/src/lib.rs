#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.is_empty() {
            None
        } else {
            Some(*self.scores.last().unwrap())
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_scores = self.scores.clone(); // Clone to avoid modifying the original
        sorted_scores.sort_by(|a, b| b.cmp(a)); // Sort descending (highest first)
        sorted_scores.into_iter().take(3).collect() // Take up to 3 and collect into Vec
    }
}

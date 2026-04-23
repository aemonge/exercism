#[derive(Debug)]
pub struct HighScores {
    // scores: [u32], -> Must be known at compilation time the arrays
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: Vec::from(scores),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores
            .last()
            .copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores
            .iter()
            .max()
            .copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self
            .scores
            .clone();
        scores.sort();
        scores.reverse();
        scores.truncate(3);
        scores
    }
}

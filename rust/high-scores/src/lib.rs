#[derive(Debug)]
pub struct HighScores<'a>{
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        return Self{
            scores: scores,
        };
    }

    pub fn scores(&self) -> &[u32] {
        return self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut result_vector = self.scores.to_vec();
        result_vector.sort_unstable_by(|a, b| b.cmp(a));
        result_vector.truncate(3);
        result_vector
    }
}

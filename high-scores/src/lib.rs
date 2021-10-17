#[derive(Debug)]
pub struct HighScores<'a> { scores: &'a[u32] }

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a[u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.len() <= 0 {
            return None;
        }
        Some(self.scores[self.scores.len() - 1])
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.len() <= 0 {
            return None;
        }
        Some(self.scores.iter().fold(0, |acc, score| {
            if *score > acc {
                *score
            } else {
                acc
            }
        }))
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_scores = self.scores.to_vec();
        sorted_scores.sort();
        sorted_scores.reverse();

        if sorted_scores.len() < 3 {
            return sorted_scores[0..sorted_scores.len()].to_owned();
        }

        sorted_scores[0..3].to_owned()
    }
}

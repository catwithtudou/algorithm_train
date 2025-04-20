use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();
        for x in answers {
            *cnt.entry(x).or_insert(0) += 1;
        }
        cnt.into_iter()
            .map(|(x, c)| (c + x) / (x + 1) * (x + 1))
            .sum()
    }
}

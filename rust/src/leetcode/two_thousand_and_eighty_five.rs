use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut word_map = HashMap::new();
        words1
            .into_iter()
            .for_each(|word| *word_map.entry(word).or_insert(0) += 1);
        words2
            .into_iter()
            .for_each(|word| {
                if let Some(num) = word_map.get(&word) {
                    if *num <= 1 {
                        *word_map.entry(word).or_insert(0) -= 1;
                    }
                }
            });

        let mut ans = 0;
        word_map.into_iter().for_each(|(_, num)| {
            if num == 0 {
                ans += 1;
            }
        });
        ans
    }
}
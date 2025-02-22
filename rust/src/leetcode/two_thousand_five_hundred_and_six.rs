use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut cnt = HashMap::new();

        for s in words {
            let mut mask = 0;
            for c in s.as_bytes() {
                mask |= 1 << (c - b'a');
            }
            ans += *cnt.get(&mask).unwrap_or(&0);
            *cnt.entry(mask).or_insert(0) += 1;
        }

        ans
    }
}

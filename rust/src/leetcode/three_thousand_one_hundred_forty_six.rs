pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut char_index = HashMap::new();
        for (i,c) in s.chars().enumerate() {
            char_index.insert(c,i as i32);
        }
        t.chars().enumerate().map(|(i,c)| (i as i32 - char_index[&c]).abs()).sum()
    }
}
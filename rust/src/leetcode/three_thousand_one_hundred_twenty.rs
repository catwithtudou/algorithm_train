pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let s: HashSet<char> = word.chars().collect();

        let mut ans = 0;

        for c in 'a'..='z' {
            if s.contains(&c) && s.contains(&c.to_uppercase().next().unwrap()) {
                ans += 1;
            }
        }

        ans
    }
}

use std::collections::HashSet;

pub struct Solution;


impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let d_map: HashSet<String> = dictionary.into_iter().collect();
        let n = s.len();
        let mut dp = vec![0; n + 1];
        for i in 1..=n {
            dp[i] = dp[i - 1] + 1;
            for j in 0..=i {
                if d_map.contains(s.get(j..i).unwrap()) {
                    dp[i] = dp[j].min(dp[i]);
                }
            }
        }
        dp[n]
    }
}
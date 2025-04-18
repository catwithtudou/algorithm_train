use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut ans = n * (n - 1) / 2;
        let mut cnt = HashMap::new();
        for i in 0..n {
            let x = nums[i] - i as i32;
            ans -= cnt.get(&x).unwrap_or(&0);
            *cnt.entry(x).or_insert(0) += 1;
        }
        ans as i64
    }
}

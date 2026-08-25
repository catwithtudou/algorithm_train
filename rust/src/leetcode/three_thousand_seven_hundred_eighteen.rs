

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let has: HashSet<i32> = nums.into_iter().collect();
        let mut multiple = k;

        while has.contains(&multiple) {
            multiple += k;
        }

        multiple
    }
}
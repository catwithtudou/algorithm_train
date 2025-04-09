use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let min = *nums.iter().min().unwrap();
        if k > min {
            return -1;
        }
        let set = nums.into_iter().collect::<HashSet<_>>();
        set.len() as i32 - if k == min { 1 } else { 0 }
    }
}

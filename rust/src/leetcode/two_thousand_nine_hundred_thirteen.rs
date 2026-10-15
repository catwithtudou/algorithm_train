use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res =0;
        for i in 0..n {
            let mut s = HashSet::new();
            for j in i..n {
                s.insert(nums[j]);
                res+= (s.len() * s.len()) as i32
            }
        }
        res
    }
}
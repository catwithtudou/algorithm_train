pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let min_value = *nums.iter().min().unwrap();
        let max_value = *nums.iter().max().unwrap();

        let values: HashSet<i32> = nums.into_iter().collect();

        (min_value + 1..max_value)
            .filter(|x| !values.contains(x))
            .collect()
    }
}
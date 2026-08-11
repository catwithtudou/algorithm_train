use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] + 1 {
                break;
            }
            sum += nums[i]
        }

        let set: HashSet<i32> = nums.into_iter().collect();

        while set.contains(&sum) {
            sum += 1;
        }

        sum
    }
}

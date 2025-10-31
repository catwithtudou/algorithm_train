use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        let mut num_map = HashMap::new();
        for &x in nums.iter() {
            *num_map.entry(x).or_insert(0) += 1;
        }

        for (x, &cnt) in num_map.iter() {
            if cnt == 2 {
                ans.push(*x);
            }
        }

        ans
    }
}

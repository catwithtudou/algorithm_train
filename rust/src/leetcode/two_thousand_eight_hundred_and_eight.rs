use std::collections::HashMap;

pub struct Solution;


impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut mp = HashMap::new();
        for i in 0..n {
            mp.entry(nums[i]).or_insert(vec![]).push(i);
        }

        mp.into_values().map(|pos| {
            let len = pos.len();
            let mut mx = pos[0] + n - pos.last().unwrap();
            for i in 1..len {
                mx = mx.max(pos[i] - pos[i - 1]);
            }
            mx
        }).min().map(|dis| dis / 2).unwrap() as i32
    }
}
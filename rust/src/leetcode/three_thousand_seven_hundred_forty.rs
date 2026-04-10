use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut pos = HashMap::<i32, Vec<i32>>::new();

        for i in 0..nums.len() {
            pos.entry(nums[i]).or_default().push(i as i32);
        }
        let mut ans = i32::MAX;
        for (_, p) in pos {
            let mut i = 2;
            while i < p.len() {
                ans = ans.min((p[i] - p[i - 2]) * 2);
                i += 1;
            }
        }

        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
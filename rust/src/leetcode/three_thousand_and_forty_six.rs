﻿pub struct Solution;


use std::collections::HashMap;

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut cnt = HashMap::new();
        for x in nums {
            *cnt.entry(x).or_insert(0) += 1;
            if cnt[&x] > 2 {
                return false;
            }
        }
        true
    }
}

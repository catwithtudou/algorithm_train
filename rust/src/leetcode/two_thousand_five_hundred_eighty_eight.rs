pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut s = 0;
        let mut ans = 0;
        let mut cnt = HashMap::with_capacity(nums.len() + 1);
        cnt.insert(0, 1);
        for x in nums {
            s ^= x;
            ans += cnt.get(&s).unwrap_or(&0);
            *cnt.entry(s).or_insert(0) += 1;
        }
        ans
    }
}

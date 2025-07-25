use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut mx = i32::MIN;
        let mut set = HashSet::new();

        for x in nums {
            if x < 0 {
                mx = mx.max(x);
            } else if set.insert(x) {
                ans += x;
            }
        }
        if set.is_empty() {
            mx
        } else {
            ans
        }
    }
}

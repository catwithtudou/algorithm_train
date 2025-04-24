use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let k = nums.iter().collect::<HashSet<_>>().len();
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = HashMap::new();
        for &x in &nums {
            *cnt.entry(x).or_insert(0) += 1;
            while cnt.len() == k {
                let out = nums[left];
                let e = cnt.get_mut(&out).unwrap();
                *e -= 1;
                if *e == 0 {
                    cnt.remove(&out);
                }
                left += 1;
            }
            ans += left;
        }
        ans as _
    }
}

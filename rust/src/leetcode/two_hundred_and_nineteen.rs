use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut set = HashSet::new();
        for (i, &x) in nums.iter().enumerate() {
            if !set.insert(x) {
                return true;
            }
            if i >= k {
                set.remove(&nums[i - k]);
            }
        }
        false
    }
}

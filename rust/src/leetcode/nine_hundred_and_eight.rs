pub struct Solution;

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let mn = *nums.iter().min().unwrap();
        let mx = *nums.iter().max().unwrap();
        return 0.max(mx-mn-2*k);
    }
}
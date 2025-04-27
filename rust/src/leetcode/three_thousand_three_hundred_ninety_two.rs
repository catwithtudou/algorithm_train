pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        (2..nums.len()).filter(|&i| (nums[i-2]+nums[i])*2 == nums[i-1]).count() as _
    }
}
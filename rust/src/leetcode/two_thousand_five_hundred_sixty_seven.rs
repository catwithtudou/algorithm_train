pub struct Solution;

impl Solution {
    pub fn minimize_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        (nums[n-3]-nums[0]).min(nums[n-2]-nums[1]).min(nums[n-1]-nums[2])
    }
}
pub struct Solution;

impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        (nums[n - 3] * nums[n - 2] * nums[n - 1]).max(nums[0] * nums[1] * nums[n - 1])
    }
}

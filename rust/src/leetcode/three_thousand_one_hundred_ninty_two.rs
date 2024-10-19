pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0]^1;
        for i in 1..nums.len() {
            ans+= nums[i]^nums[i-1]
        }

        return ans
    }
}
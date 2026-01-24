pub struct Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        nums.sort_unstable();

        let n = nums.len();

        for i in 0..n / 2 {
            ans = ans.max(nums[i] + nums[n - 1 - i]);
        }

        ans
    }
}
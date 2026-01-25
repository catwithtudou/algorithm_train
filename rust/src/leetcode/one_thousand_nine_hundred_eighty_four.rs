pub struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut ans = i32::MAX;
        for i in k as usize - 1..nums.len() {
            ans = ans.min(nums[i] - nums[i - (k as usize - 1)]);
        }
        ans
    }
}

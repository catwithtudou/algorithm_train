pub struct Solution;

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut nums: Vec<i64> = nums
            .into_iter()
            .map(|x| x as i64 * x as i64)
            .collect();

        nums.sort_unstable();

        let mid = nums.len() / 2;

        nums[mid..].iter().sum::<i64>()
            - nums[..mid].iter().sum::<i64>()
    }
}
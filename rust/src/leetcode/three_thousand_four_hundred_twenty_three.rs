pub struct Solution;

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        (0..nums.len())
            .map(|i| (nums[(i + 1) % nums.len()] - nums[i]).abs())
            .max()
            .unwrap_or(0)
    }
}

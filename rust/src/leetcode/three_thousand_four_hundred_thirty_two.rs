pub struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let s = nums.iter().sum::<i32>();
        if s % 2 != 0 {
            0
        } else {
            (nums.len() - 1) as i32
        }
    }
}

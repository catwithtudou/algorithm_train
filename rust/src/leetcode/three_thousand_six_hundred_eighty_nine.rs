pub struct Solution;

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let mx = *nums.iter().max().unwrap();
        let mi = *nums.iter().min().unwrap();

        (mx- mi) as i64 * k as i64
    }
}
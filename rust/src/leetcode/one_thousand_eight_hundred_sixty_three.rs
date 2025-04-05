pub struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.into_iter().reduce(|or, x| or | x).unwrap() << (n - 1)
    }
}

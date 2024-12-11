pub struct Solution;

impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let p = nums.iter().position(|&x| x == 1).unwrap();
        let q = nums.iter().position(|&x| x == n as i32).unwrap();
        (p + n - 1 - q - (p > q) as usize) as _
    }
}

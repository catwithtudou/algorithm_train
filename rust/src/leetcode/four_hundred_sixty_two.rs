pub struct Solution;

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let medium = *nums.select_nth_unstable(n / 2).1;

        nums.into_iter().map(|x| (x - medium).abs()).sum()
     }
}
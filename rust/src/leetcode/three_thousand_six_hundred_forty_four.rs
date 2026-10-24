pub struct Solution;

impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        for (i, &x) in nums.iter().enumerate() {
            if i != x as usize {
                ans &= x;
            }
        }
        ans.max(0)
    }
}
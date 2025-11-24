pub struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut ans = vec![false; nums.len()];
        let mut pre = 0;
        for (i, num) in nums.iter().enumerate() {
            pre = (pre << 1 | num) % 5;
            ans[i] = pre == 0;
        }
        ans
    }
}

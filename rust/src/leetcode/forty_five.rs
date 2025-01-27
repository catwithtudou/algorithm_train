pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut ans, mut cur_r, mut next_r) = (0, 0, 0);
        for (i, &num) in nums.iter().enumerate().take(nums.len() - 1) {
            next_r = next_r.max(i + num as usize);
            if i == cur_r {
                cur_r = next_r;
                ans += 1;
            }
        }
        ans
    }
}

pub struct Solution;

impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();

        let mut suf = vec![0; n];

        for i in (0..n - 1).rev() {
            suf[i] = suf[i + 1] | nums[i + 1];
        }

        let (mut pre, mut ans) = (0, 0);

        for (i, &x) in nums.iter().enumerate() {
            ans = ans.max(pre | (x as i64) << k | suf[i] as i64);
            pre = pre | x as i64;
        }

        ans
    }
}

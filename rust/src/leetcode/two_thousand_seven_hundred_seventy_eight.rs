pub struct Solution;

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut i = 1;

        while i * i <= n {
            if n % i == 0 {
                ans += nums[i - 1] * nums[i - 1];

                // 避免 n 是完全平方数时重复统计
                if i * i < n {
                    let j = n / i;
                    ans += nums[j - 1] * nums[j - 1];
                }
            }

            i += 1;
        }

        ans
    }
}
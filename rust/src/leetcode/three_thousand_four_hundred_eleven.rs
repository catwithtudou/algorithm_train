pub struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while a != 0 {
                (a, b) = (b % a, a);
            }
            b
        }

        let (mut ans, mut mul, mut left) = (2usize, 1i32, 0usize);
        for (right, &x) in nums.iter().enumerate() {
            while gcd(mul, x) > 1 {
                mul /= nums[left];
                left += 1;
            }
            mul *= x;
            ans = ans.max(right - left + 1);
        }
        ans as i32
    }
}
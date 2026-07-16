pub struct Solution;

impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while a != 0 {
                (a, b) = (b % a, a);
            }
            b
        }

        let n = nums.len();
        let mut prefix_gcds = Vec::with_capacity(n);
        let mut max_value = 0;

        for x in nums {
            max_value = max_value.max(x);
            prefix_gcds.push(gcd(x, max_value));
        }

        prefix_gcds.sort_unstable();

        (0..n / 2)
            .map(|i| gcd(prefix_gcds[i], prefix_gcds[n - 1 - i]) as i64)
            .sum()
    }
}
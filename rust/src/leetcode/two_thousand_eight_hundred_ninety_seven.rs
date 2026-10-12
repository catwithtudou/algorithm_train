pub struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut cnt = [0i32; 30];

        for x in nums {
            for (i, c) in cnt.iter_mut().enumerate() {
                *c += (x >> i) & 1;
            }
        }

        let mut ans = 0i64;

        for _ in 0..k {
            let mut x = 0i64;

            for (i, c) in cnt.iter_mut().enumerate() {
                if *c > 0 {
                    *c -= 1;
                    x |= 1i64 << i;
                }
            }

            ans = (ans + x * x) % MOD;
        }

        ans as i32
    }
}

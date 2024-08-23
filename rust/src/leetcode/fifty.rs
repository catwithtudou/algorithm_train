pub struct Solution;

impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        let mut n = n as i64;
        let mut ans = 1.0;
        if n < 0 {
            n = -n;
            x = 1.0 / x;
        }
        while n > 0 {
            if (n & 1) == 1 {
                ans *= x;
            }
            x *= x;
            n >>= 1;
        }

        ans
    }
}

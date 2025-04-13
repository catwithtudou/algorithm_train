pub struct Solution;

impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn count_good_numbers(n: i64) -> i32 {
        (Self::quick_pow(5, (n+1)/2) * Self::quick_pow(4, n/2) % Self::MOD) as i32
    }

    fn quick_pow(mut x: i64, mut n: i64) -> i64 {
        let mut res = 1;
        while n > 0 {
            if n & 1 > 0 {
                res = res * x % Self::MOD;
            }
            x = x * x % Self::MOD;
            n >>= 1;
        }
        res
    }
}

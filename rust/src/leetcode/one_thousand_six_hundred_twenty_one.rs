pub struct Solution;

impl Solution {
    fn power(mut a: i64, mut e: i64) -> i64 {
        const MOD: i64 = 1_000_000_007;
        let mut result = 1;
        while e > 0 {
            if e & 1 == 1 { result = result * a % MOD; }
            a = a * a % MOD;
            e >>= 1;
        }
        result
    }

    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut numerator = 1i64;
        let mut denominator = 1i64;
        for i in 1..=2 * k {
            numerator = numerator * (n + k - i) as i64 % MOD;
            denominator = denominator * i as i64 % MOD;
        }
        (numerator * Self::power(denominator, MOD - 2) % MOD) as i32
    }
}
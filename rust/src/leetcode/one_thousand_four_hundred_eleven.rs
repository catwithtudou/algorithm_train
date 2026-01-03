pub struct Solution;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = n as usize;
        let mut f = vec![0; n + 1];
        f[0] = 3;
        f[1] = 12;
        for i in 2..=n {
            f[i] = ((f[i - 1] as i64 * 5 - f[i - 2] as i64 * 2) % MOD as i64) as i32;
        }

        (f[n] % MOD + MOD) % MOD
    }
}

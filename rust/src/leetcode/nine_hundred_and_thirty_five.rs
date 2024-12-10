pub struct Solution;

use lazy_static::lazy_static;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        lazy_static! {
            static ref F: Vec<[i32; 4]> = {
                let mut f = vec![[0; 4]; 5000];
                f[0] = [1, 1, 1, 1];
                for i in 1..5000 {
                    f[i][0] = (f[i - 1][1] + f[i - 1][2]) % MOD;
                    f[i][1] = (f[i - 1][0] * 2) % MOD;
                    f[i][2] = (((f[i - 1][0] * 2) % MOD) + f[i - 1][3]) % MOD;
                    f[i][3] = (f[i - 1][2] * 2) % MOD;
                }
                f
            };
        }

        if n == 1 {
            return 10;
        }

        let n = n as usize;
        let result = ((((F[n - 1][0] as i64 * 4) % MOD as i64)
            + ((F[n - 1][1] as i64 * 2) % MOD as i64)
            + ((F[n - 1][2] as i64 * 2) % MOD as i64)
            + F[n - 1][3] as i64)
            % MOD as i64) as i32;

        result
    }
}

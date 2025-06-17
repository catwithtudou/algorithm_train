pub struct Solution;

impl Solution {
    const MOD: i64 = 1_000_000_007;
    const MX: usize = 100_000;

    pub fn quick_pow(x: i64, n: i64, mod_val: i64) -> i64 {
        let mut ans = 1i64;
        let mut x = x;
        let mut n = n;

        while n > 0 {
            if n % 2 > 0 {
                ans = ans * x % mod_val;
            }
            x = x * x % mod_val;
            n /= 2;
        }
        ans
    }

    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let mut f = [0i64; Self::MX];
        let mut inv_f = [0i64; Self::MX];

        // 计算阶乘
        f[0] = 1;
        for i in 1..Self::MX {
            f[i] = f[i - 1] * i as i64 % Self::MOD;
        }

        // 计算阶乘的逆元
        inv_f[Self::MX - 1] = Self::quick_pow(f[Self::MX - 1], Self::MOD - 2, Self::MOD);
        for i in (1..Self::MX).rev() {
            inv_f[i - 1] = inv_f[i] * i as i64 % Self::MOD;
        }

        // 组合数计算函数
        let comb = |n: usize, m: usize| -> i64 { f[n] * inv_f[m] % Self::MOD * inv_f[n - m] % Self::MOD };

        let result = comb((n - 1) as usize, k as usize) * m as i64 % Self::MOD
            * Self::quick_pow(m as i64 - 1, (n - k - 1) as i64, Self::MOD)
            % Self::MOD;

        result as i32
    }
}

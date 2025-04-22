pub struct Solution;

impl Solution {
    const MAX_N: i32 = 10_000;
    const MAX_E: i32 = 13;
    const MOD: i64 = 1_000_000_007;

    // 预计算所有需要的组合数
    fn calculate_combinations(n: i32) -> Vec<Vec<i64>> {
        let size = (n + Self::MAX_E) as usize;
        let mut c = vec![vec![0i64; (Self::MAX_E + 1) as usize]; size as usize];

        // 初始化组合数数组
        for i in 0..size {
            c[i][0] = 1;
            for j in 1..=i.min(Self::MAX_E as usize) {
                c[i][j] = (c[i - 1][j] + c[i - 1][j - 1]) % Self::MOD;
            }
        }
        c
    }

    // 预计算所有数的质因数指数
    fn precompute_exponents(max_value: i32) -> Vec<Vec<i32>> {
        let mut exp = vec![Vec::new(); (max_value + 1) as usize];

        for x in 2..=max_value {
            let mut t = x;
            let mut i = 2;
            while i * i <= t {
                let mut e = 0;
                while t % i == 0 {
                    e += 1;
                    t /= i;
                }
                if e > 0 {
                    exp[x as usize].push(e);
                }
                i += 1;
            }
            if t > 1 {
                exp[x as usize].push(1);
            }
        }
        exp
    }

    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        // 预计算组合数和质因数指数
        let combinations = Self::calculate_combinations(n);
        let exponents = Self::precompute_exponents(max_value);

        let mut ans: i64 = 0;

        // 对于每个可能的值
        for x in 1..=max_value {
            let mut res: i64 = 1;
            // 使用预计算的质因数指数
            for &e in exponents[x as usize].iter() {
                // 使用预计算的组合数
                res = (res * combinations[(e + n - 1) as usize][e as usize]) % Self::MOD;
            }
            ans = (ans + res) % Self::MOD;
        }

        ans as i32
    }
}

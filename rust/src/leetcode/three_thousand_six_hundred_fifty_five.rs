pub struct Solution;

impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let n = nums.len();
        let t = (n as f64).sqrt() as usize;

        let mut groups: Vec<Vec<(usize, usize, i64)>> = vec![vec![]; t];
        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let k = q[2] as usize;
            let v = q[3] as i64;
            if k < t {
                groups[k].push((l, r, v));
            } else {
                let mut i = l;
                while i <= r {
                    nums[i] = nums[i] * v % MOD;
                    i += k;
                }
            }
        }

        let mut dif = vec![1; n + t];
        for k in 1..t {
            if groups[k].is_empty() {
                continue;
            }
            dif.fill(1);
            for &(l, r, v) in &groups[k] {
                dif[l] = dif[l] * v % MOD;
                let r_idx = ((r - l) / k + 1) * k + l;
                dif[r_idx] = dif[r_idx] * Self::pow_mod(v, MOD - 2, MOD) % MOD;
            }

            for i in k..n {
                dif[i] = dif[i] * dif[i - k] % MOD;
            }
            for i in 0..n {
                nums[i] = nums[i] * dif[i] % MOD;
            }
        }

        nums.into_iter().fold(0, |acc, x| acc ^ x as i32)
    }

    fn pow_mod(mut x: i64, mut y: i64, m: i64) -> i64 {
        let mut res = 1;
        while y > 0 {
            if y & 1 == 1 {
                res = res * x % m;
            }
            x = x * x % m;
            y >>= 1;
        }
        res
    }
}
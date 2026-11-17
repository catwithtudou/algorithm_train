pub struct Solution;

impl Solution {
    pub fn minimum_changes(s: String, k: i32) -> i32 {
        fn calc(s: &[u8], divisors: &[Vec<usize>]) -> i32 {
            let n = s.len();
            let mut res = n as i32;

            for &d in &divisors[n] {
                let mut count = 0;

                for offset in 0..d {
                    let mut i = offset;
                    let mut j = n - d + offset;

                    while i < j {
                        if s[i] != s[j] {
                            count += 1;
                        }
                        i += d;
                        j -= d;
                    }
                }

                res = res.min(count);
            }

            res
        }

        let s = s.as_bytes();
        let n = s.len();
        let k = k as usize;

        // 预处理每个长度的真约数
        let mut divisors = vec![Vec::new(); n + 1];

        for d in 1..=n / 2 {
            for j in (d * 2..=n).step_by(d) {
                divisors[j].push(d);
            }
        }

        // modify[l][r]: 将 s[l..=r] 修改成半回文串的最小次数
        let mut modify = vec![vec![0i32; n]; n];

        for l in 0..n {
            for r in l + 1..n {
                modify[l][r] = calc(&s[l..=r], &divisors);
            }
        }

        // dp[j]: 将 s[0..=j] 划分为当前数量的子串的最小代价
        let mut dp = modify[0].clone();

        for i in 1..k {
            let start = i * 2 + 1;
            let end = n - 1 - (k - 1 - i) * 2;

            // 必须倒序更新，避免覆盖上一轮的状态
            for j in (start..=end).rev() {
                dp[j] = n as i32;

                for l in i * 2..j {
                    dp[j] = dp[j].min(
                        dp[l - 1] + modify[l][j]
                    );
                }
            }
        }

        dp[n - 1]
    }
}
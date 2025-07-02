pub struct Solution;

impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        let k = k as usize;
        if word.len() < k {
            return 0;
        }

        const MOD: i32 = 1_000_000_007;

        let mut cnts = Vec::new();
        let mut ans = 1i32;
        let mut cnt = 0;
        let mut k_remaining = k;

        let word_bytes = word.as_bytes();

        for i in 0..word.len() {
            cnt += 1;
            if i == word.len() - 1 || word_bytes[i] != word_bytes[i + 1] {
                if cnt > 1 {
                    if k_remaining > 0 {
                        cnts.push(cnt - 1);
                    }
                    ans = (ans as i64 * cnt as i64 % MOD as i64) as i32;
                }
                k_remaining = k_remaining.saturating_sub(1);
                cnt = 0;
            }
        }

        if k_remaining == 0 {
            return ans;
        }

        let m = cnts.len();
        let mut f = vec![vec![0i32; k_remaining]; m + 1];

        // 初始化 f[0][i] = 1
        for i in 0..k_remaining {
            f[0][i] = 1;
        }

        let mut s = vec![0i32; k_remaining + 1];

        for (i, &c) in cnts.iter().enumerate() {
            // 计算前缀和
            for j in 0..k_remaining {
                s[j + 1] = (s[j] + f[i][j]) % MOD;
            }

            // 更新 f[i+1][j]
            for j in 0..k_remaining {
                let left_bound = if j >= c { j - c } else { 0 };
                f[i + 1][j] = (s[j + 1] - s[left_bound] + MOD) % MOD;
            }
        }

        (ans as i64 - f[m][k_remaining - 1] as i64 + MOD as i64) as i32 % MOD
    }
}

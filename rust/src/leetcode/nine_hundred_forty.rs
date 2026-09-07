pub struct Solution;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let bytes = s.as_bytes();
        let n = bytes.len();

        // dp[i][c]:
        // 使用 s 的前 i 个字符，且最后一个字符是 c 的不同非空子序列数量
        let mut dp = vec![[0_i64; 26]; n + 1];

        for i in 0..n {
            let total = dp[i].iter().sum::<i64>() % MOD;

            dp[i + 1] = dp[i];

            let c = (bytes[i] - b'a') as usize;

            // 所有旧子序列后面加上当前字符
            // 再加上只包含当前字符本身的子序列
            dp[i + 1][c] = (total + 1) % MOD;
        }

        (dp[n].iter().sum::<i64>() % MOD) as i32
    }
}
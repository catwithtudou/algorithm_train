pub struct Solution;

impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;

        let n = s.len();
        let bytes = s.as_bytes();

        // pow10[i] = 10^i % MOD
        let mut pow10 = vec![1_i64; n + 1];
        for i in 1..=n {
            pow10[i] = pow10[i - 1] * 10 % MOD;
        }

        // sum_d[i]：前 i 个字符的数字和
        let mut sum_d = vec![0_i64; n + 1];

        // pre_num[i]：前 i 个字符中，去掉 0 后组成的数字
        let mut pre_num = vec![0_i64; n + 1];

        // sum_non_zero[i]：前 i 个字符中非 0 数字的数量
        let mut sum_non_zero = vec![0_usize; n + 1];

        for i in 0..n {
            let d = (bytes[i] - b'0') as i64;

            sum_d[i + 1] = sum_d[i] + d;
            pre_num[i + 1] = pre_num[i];
            sum_non_zero[i + 1] = sum_non_zero[i];

            if d > 0 {
                pre_num[i + 1] = (pre_num[i] * 10 + d) % MOD;
                sum_non_zero[i + 1] += 1;
            }
        }

        let mut ans = Vec::with_capacity(queries.len());

        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize + 1;

            let len = sum_non_zero[r] - sum_non_zero[l];

            // 取出 s[l..r] 中去掉 0 后形成的数字
            let x = (pre_num[r] - pre_num[l] * pow10[len] % MOD + MOD) % MOD;

            // s[l..r] 的数字和
            let digit_sum = sum_d[r] - sum_d[l];

            ans.push((x * digit_sum % MOD) as i32);
        }

        ans
    }
}
pub struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut ans: i64 = 0;

        for i in 1..=n {
            // 计算 i 的二进制位数
            let w = 32 - (i as u32).leading_zeros();
            // 左移 w 位，然后与 i 进行或运算，最后取模
            ans = ((ans << w) | (i as i64)) % MOD;
        }

        ans as i32
    }
}

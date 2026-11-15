pub struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut cnt = [1_i64, 0]; // 初始前缀和为 0（偶数），所以 cnt[0] = 1
        let mut s = 0_usize;
        let mut ans = 0_i64;

        for x in arr {
            s = (s + x as usize) % 2;
            ans += cnt[s ^ 1];
            cnt[s] += 1;
        }

        (ans % MOD) as i32
    }
}
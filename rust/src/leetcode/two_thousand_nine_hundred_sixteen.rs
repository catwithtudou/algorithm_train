pub struct Solution;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();
        let mut t1 = vec![0i64; n + 1]; // 差分 d[i]
        let mut t2 = vec![0i64; n + 1]; // i * d[i]

        fn upd(t1: &mut [i64], t2: &mut [i64], n: usize, mut i: usize, v: i64) {
            let w = i as i64 * v;
            while i <= n {
                t1[i] += v;
                t2[i] += w;
                i += i & i.wrapping_neg();
            }
        }
        fn pre(t1: &[i64], t2: &[i64], p: usize) -> i64 {
            let (mut s1, mut s2, mut i) = (0i64, 0i64, p);
            while i > 0 {
                s1 += t1[i];
                s2 += t2[i];
                i -= i & i.wrapping_neg();
            }
            (p as i64 + 1) * s1 - s2
        }

        let mut last = vec![0usize; 100_001]; // 值 -> 上次出现位置，0 表示没出现
        let (mut ans, mut s) = (0i64, 0i64);
        for r in 1..=n {
            let x = nums[r - 1] as usize;
            let j = last[x];
            let sum = pre(&t1, &t2, r) - pre(&t1, &t2, j);
            s = (s + 2 * sum + (r - j) as i64) % MOD; // 平方和增量
            upd(&mut t1, &mut t2, n, j + 1, 1);
            upd(&mut t1, &mut t2, n, r + 1, -1); // f[j+1..r] 都 +1
            ans = (ans + s) % MOD;
            last[x] = r;
        }
        ans as i32
    }
}
pub struct Solution;

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        let k = (r - l + 1) as usize;
        let mut f0 = vec![1 as i64; k];
        let mut f1 = vec![1 as i64; k];
        let mut s0 = vec![0 as i64; k + 1];
        let mut s1 = vec![0 as i64; k + 1];
        for _ in 0..n - 1 {
            for i in 0..k {
                s0[i + 1] = s0[i] + f0[i];
                s1[i + 1] = s1[i] + f1[i];
            }
            for i in 0..k {
                f0[i] = s1[i] % 1_000_000_007;
                f1[i] = (s0[k as usize] - s0[i + 1]) % 1_000_000_007;
            }
        }
        let mut ans = 0;
        for i in 0..k {
            ans += f0[i] + f1[i];
        }
        (ans % 1_000_000_007) as _
    }
}
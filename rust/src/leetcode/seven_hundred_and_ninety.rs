pub struct Solution;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let n = n as usize;
        let mut f = vec![0; n + 1];
        f[0] = 1;
        f[1] = 1;
        f[2] = 2;
        for i in 3..=n {
            let a = (f[i - 1] as i64 * 2) % 1000000007;
            let b = f[i - 3] as i64;
            f[i] = ((a + b) % 1000000007) as i32;
        }
        f[n]
    }
}

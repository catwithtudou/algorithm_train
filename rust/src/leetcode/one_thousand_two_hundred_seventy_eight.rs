pub struct Solution;

impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();
        let k = k as usize;

        let mut change = vec![vec![0; n]; n];
        for i in (0..=n - 1).rev() {
            for j in (i + 1)..n {
                change[i][j] = change[i + 1][j - 1];
                if s_bytes[i] != s_bytes[j] {
                    change[i][j] += 1;
                }
            }
        }

        let mut f = vec![vec![0; n]; k];
        f[0] = change[0].clone();
        for i in 1..k {
            for r in i..=(n - k + i) {
                f[i][r] = i32::MAX;
                for l in i..=r {
                    f[i][r] = f[i][r].min(f[i - 1][l - 1] + change[l][r]);
                }
            }
        }
        f[k - 1][n - 1]
    }
}

pub struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let (n, m) = (s1.len(), s2.len());
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        let mut total = 0;
        for &c in s1 {
            total += c as i32;
        }
        for &c in s2 {
            total += c as i32;
        }

        let mut f = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                if s1[i] == s2[j] {
                    f[i + 1][j + 1] = f[i][j] + s1[i] as i32;
                } else {
                    f[i + 1][j + 1] = f[i][j + 1].max(f[i + 1][j]);
                }
            }
        }

        total - 2 * f[n][m]
    }
}

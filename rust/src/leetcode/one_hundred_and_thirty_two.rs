pub struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();

        let mut is_palindrome = vec![vec![true; n]; n];
        for l in (0..n - 1).rev() {
            for r in l + 1..n {
                is_palindrome[l][r] = s_bytes[l] == s_bytes[r] && is_palindrome[l + 1][r - 1];
            }
        }

        let mut f = vec![0; n];

        for r in 0..n {
            if is_palindrome[0][r] {
                continue;
            }
            let mut res = i32::MAX;
            for l in 1..=r {
                if is_palindrome[l][r] {
                    res = res.min(f[l - 1] + 1);
                }
            }
            f[r]=res;
        }

        f[n - 1]
    }
}

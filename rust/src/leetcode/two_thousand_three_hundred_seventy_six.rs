pub struct Solution;

impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {
        fn dfs(
            i: usize,
            mask: usize,
            is_limit: bool,
            is_num: bool,
            s: &[u8],
            memo: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if i == s.len() {
                return if is_num { 1 } else { 0 };
            }
            if !is_limit && is_num && memo[i][mask] != -1 {
                return memo[i][mask];
            }
            let mut res = 0;
            if !is_num {
                res = dfs(i + 1, mask, false, false, s, memo);
            }
            let up = if is_limit { s[i] - b'0' } else { 9 };
            let low = if is_num { 0 } else { 1 };
            for d in low..=up {
                if (mask << d & 1) == 0 {
                    res += dfs(i + 1, mask | (1 << d), is_limit && d == up, true, s, memo);
                }
            }
            if !is_limit && is_num {
                memo[i][mask] = res;
            }

            return res;
        }
        let s = n.to_string();
        let s = s.as_bytes();
        let mut memo = vec![vec![-1; 1 << 10]; s.len()];
        return dfs(0, 0, true, false, &s, &mut memo);
    }
}

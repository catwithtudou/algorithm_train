pub struct Solution;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        use std::collections::HashSet;

        fn dfs(
            s: &[u8],
            i: usize,
            balance: usize,
            left: usize,
            right: usize,
            path: &mut String,
            ans: &mut HashSet<String>,
        ) {
            // 剩余字符必须足够完成删除，并闭合已保留的左括号。
            if s.len() - i < left + right + balance {
                return;
            }
            if i == s.len() {
                ans.insert(path.clone());
                return;
            }
            let ch = s[i];
            if ch == b'(' && left > 0 {
                dfs(s, i + 1, balance, left - 1, right, path, ans);
            }
            if ch == b')' && right > 0 {
                dfs(s, i + 1, balance, left, right - 1, path, ans);
            }
            if ch == b')' && balance == 0 {
                return;
            }
            let balance = match ch {
                b'(' => balance + 1,
                b')' => balance - 1,
                _ => balance,
            };
            path.push(ch as char);
            dfs(s, i + 1, balance, left, right, path, ans);
            path.pop();
        }

        let (mut left, mut right) = (0, 0);
        for ch in s.bytes() {
            if ch == b'(' {
                left += 1;
            } else if ch == b')' {
                if left > 0 {
                    left -= 1;
                } else {
                    right += 1;
                }
            }
        }
        let mut ans = HashSet::new();
        let mut path = String::with_capacity(s.len());
        dfs(s.as_bytes(), 0, 0, left, right, &mut path, &mut ans);
        ans.into_iter().collect()
    }
}

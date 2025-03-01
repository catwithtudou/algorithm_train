pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();
        let mut ans: Vec<Vec<String>> = Vec::new();
        let mut path: Vec<String> = Vec::new();

        Self::dfs(0, s_bytes, n, &mut path, &mut ans);
        ans
    }

    fn dfs(i: usize, s_bytes: &[u8], n: usize, path: &mut Vec<String>, ans: &mut Vec<Vec<String>>) {
        if i == n {
            ans.push(path.clone());
            return;
        }

        for j in i..n {
            if Self::is_palindrome(s_bytes, i, j) {
                path.push(String::from_utf8(s_bytes[i..=j].to_vec()).unwrap());
                Self::dfs(j + 1, s_bytes, n, path, ans);
                path.pop();
            }
        }
    }

    fn is_palindrome(s: &[u8], mut left: usize, mut right: usize) -> bool {
        while left < right {
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

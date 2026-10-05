pub struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut ans = 0;
        let mut depth = 0;
        for (i, &ch) in bytes.iter().enumerate() {
            if ch == b'(' {
                depth += 1;
            } else {
                depth -= 1;
                if bytes[i - 1] == b'(' {
                    // 每个最内层的 () 被外层括号翻倍 depth 次。
                    ans += 1 << depth;
                }
            }
        }
        ans
    }
}

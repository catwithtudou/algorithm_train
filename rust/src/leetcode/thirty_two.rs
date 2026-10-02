pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut ans = 0;
        let mut stack = vec![-1];
        for (i, ch) in s.bytes().enumerate() {
            let i = i as i32;
            if ch == b'(' {
                stack.push(i);
            } else {
                stack.pop();
                if let Some(&left) = stack.last() {
                    ans = ans.max(i - left);
                } else {
                    // 无法匹配的右括号作为下一段的左边界。
                    stack.push(i);
                }
            }
        }
        ans
    }
}

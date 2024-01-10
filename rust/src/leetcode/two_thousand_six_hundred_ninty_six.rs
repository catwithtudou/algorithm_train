pub struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = Vec::new();
        for &c in s.as_bytes() {
            stack.push(c);
            if stack.len() < 2 {
                continue;
            }
            let m = stack.len();
            if (stack[m - 2] == b'A' && stack[m - 1] == b'B') || (stack[m - 2] == b'C' && stack[m - 1] == b'D') {
                stack.pop();
                stack.pop();
            }
        }
        stack.len() as i32
    }
}
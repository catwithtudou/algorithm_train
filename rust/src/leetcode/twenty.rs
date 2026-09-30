pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());
        for ch in s.bytes() {
            match ch {
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                _ => {
                    if stack.pop() != Some(ch) {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}

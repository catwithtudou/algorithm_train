pub struct Solution;

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut st = vec![];
        for c in s.bytes() {
            if c.is_ascii_digit() {
                st.pop();
            } else {
                st.push(c);
            }
        }
        unsafe { String::from_utf8_unchecked(st) }
    }
}

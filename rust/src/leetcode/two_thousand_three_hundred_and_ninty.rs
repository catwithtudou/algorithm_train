pub struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut st = vec![];
        for c in s.bytes() {
            if c == b'*' {
                st.pop();
            }else{
                st.push(c);
            }
        }

        unsafe {String::from_utf8_unchecked(st)}
    }
}
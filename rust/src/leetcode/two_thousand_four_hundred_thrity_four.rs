pub struct Solution;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let n = s.len();
        let mut suf_min = vec![u8::MAX; n + 1];
        for (i, ch) in s.bytes().enumerate().rev() {
            suf_min[i] = suf_min[i + 1].min(ch);
        }

        let mut ans = Vec::with_capacity(n);
        let mut st = vec![];
        for (i, ch) in s.bytes().enumerate() {
            st.push(ch);
            while let Some(&top) = st.last() {
                if top > suf_min[i + 1] {
                    break;
                }
                ans.push(st.pop().unwrap());
            }
        }

        unsafe { String::from_utf8_unchecked(ans) }
    }
}

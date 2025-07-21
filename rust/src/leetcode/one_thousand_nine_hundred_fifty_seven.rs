pub struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let s = s.into_bytes();
        let mut ans = vec![];
        let mut cnt = 0;
        for (i, &ch) in s.iter().enumerate() {
            cnt += 1;
            if cnt < 3 {
                ans.push(ch);
            }
            if i + 1 < s.len() && ch != s[i + 1] {
                cnt = 0;
            }
        }
        unsafe { String::from_utf8_unchecked(ans) }
    }
}

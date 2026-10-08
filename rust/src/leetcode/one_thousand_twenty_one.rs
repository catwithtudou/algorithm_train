pub struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut ans = vec![];
        let mut depth = 0;
        for ch in s.bytes() {
            if ch == b'(' {
                if depth > 0 {
                    ans.push(ch);
                }
                depth+=1;
            }else{
                depth-=1;
                if depth>0 {
                    ans.push(ch);
                }
            }
        }
        unsafe { String::from_utf8_unchecked(ans)}
    }
}
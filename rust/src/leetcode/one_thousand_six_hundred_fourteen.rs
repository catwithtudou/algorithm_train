pub struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut ans = 0;
        let mut depth = 0;
        for ch in s.bytes() {
            if ch == b'(' {
                depth+=1;
                ans=ans.max(depth);
            }else if ch == b')' {
                depth-=1;
            }
        }
        ans
    }
}
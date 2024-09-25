use std::str::pattern;

pub struct Solution;

impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let pattern = pattern.as_bytes();
        let (x,y) = (pattern[0],pattern[1]);
        let mut ans = 0i64;
        let (mut cnt_x,mut cnt_y) = (0,0);
        for c in text.bytes() {
            if c == y {
                ans+=cnt_x;
                cnt_y+=1;
            }
            if c == x {
                cnt_x+=1;
            }
        }
        ans + cnt_x.max(cnt_y) as i64
    }
}
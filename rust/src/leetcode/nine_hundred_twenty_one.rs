pub struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let (mut left, mut ans) = (0, 0);
        for ch in s.bytes() {
            if ch == b'(' {
                left += 1;
            } else if left > 0 {
                left -= 1;
            } else {
                // 当前右括号没有匹配的左括号，需要补一个。
                ans += 1;
            }
        }
        ans + left
    }
}

pub struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        // 未匹配左括号数量的最小值和最大值。
        let (mut low, mut high) = (0, 0);
        for ch in s.bytes() {
            match ch {
                b'(' => {
                    low += 1;
                    high += 1;
                }
                b')' => {
                    low -= 1;
                    high -= 1;
                }
                b'*' => {
                    low -= 1;
                    high += 1;
                }
                _ => unreachable!(),
            }
            if high < 0 {
                return false;
            }
            low = low.max(0);
        }
        low == 0
    }
}

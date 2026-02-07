pub struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut f = 0;
        let mut b = 0;

        for c in s.chars() {
            if c == 'b' {
                b += 1;
            } else {
                f = (f + 1).min(b);
            }
        }

        f
    }
}

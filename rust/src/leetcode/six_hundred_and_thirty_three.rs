pub struct Solution;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut a = 0;
        let mut b = (c as f64).sqrt() as i32;
        while a <= b {
            if a * a == c - b * b {
                return true;
            } else if a * a < c - b * b {
                a += 1;
            } else {
                b -= 1;
            }
        }

        false
    }
}

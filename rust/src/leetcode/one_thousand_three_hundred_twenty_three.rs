pub struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut x = num;
        let mut base = 1;
        let mut max_base = 0;
        while x > 0 {
            if x % 10 == 6 {
                max_base = base;
            }
            x /= 10;
            base *= 10;
        }
        num + max_base * 3
    }
}

pub struct Solution;

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        for i in n.. {
            let mut x = i;
            let mut product = 1;

            while x > 0 {
                product *= x % 10;
                x /= 10;
            }

            if product % t == 0 {
                return i;
            }
        }

        unreachable!()
    }
}
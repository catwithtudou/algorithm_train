pub struct Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut s = 0;
        let mut m = 1;
        let mut x = n;
        while x > 0 {
            let d = x % 10;
            s += d;
            m *= d;
            x /= 10;
        }
        n % (s + m) == 0
    }
}

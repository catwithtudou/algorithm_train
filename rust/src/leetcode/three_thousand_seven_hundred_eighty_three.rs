pub struct Solution;

impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let mut rev = 0;
        let mut x = n;
        while x > 0 {
            rev = rev * 10 + x % 10;
            x /= 10;
        }
        (n - rev).abs()
    }
}
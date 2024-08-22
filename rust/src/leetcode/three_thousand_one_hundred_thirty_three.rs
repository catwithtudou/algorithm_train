pub struct Solution;

impl Solution {
    pub fn min_end(mut n: i32, x: i32) -> i64 {
        let (mut i, mut j) = (0, 0);
        n -= 1;
        let (n, mut x) = (n as i64, x as i64);
        while n >> j > 0 {
            if (x >> i & 1) == 0 {
                x |= (n >> j & 1) << i;
                j += 1;
            }
            i += 1;
        }
        x
    }
}

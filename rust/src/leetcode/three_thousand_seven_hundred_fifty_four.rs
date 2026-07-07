pub struct Solution;

impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let mut n = n as i64;
        let mut sum = 0;
        let mut pow10 = 1;
        let mut num = 0;
        while n > 0 {
            let d = n % 10;
            if d > 0 {
                num += d * pow10;
                sum += d;
                pow10 *= 10;
            }
            n /= 10;
        }
        sum * num
    }
}

pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut prev = 0;
        let mut val = 0;
        let mut sum = 1;
        for i in 1..=n {
            prev = val;
            val = sum;
            sum = prev + val;
        }
        sum
    }
}
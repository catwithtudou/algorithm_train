pub struct Solution;

impl Solution {
    pub fn generate_key(mut num1: i32, mut num2: i32, mut num3: i32) -> i32 {
        let mut ans = 0;
        let mut pow = 1;
        while num1 > 0 && num2 > 0 && num3 > 0 {
            ans += (num1 % 10).min(num2 % 10).min(num3 % 10) * pow;
            pow *= 10;
            num1 /= 10;
            num2 /= 10;
            num3 /= 10;
        }
        ans
    }
}

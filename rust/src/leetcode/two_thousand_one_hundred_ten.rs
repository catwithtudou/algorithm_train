pub struct Solution;

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut desc = 0;
        for i in 0..prices.len() {
            if i > 0 && prices[i] == prices[i - 1] - 1 {
                desc += 1;
            } else {
                desc = 1;
            }
            ans += desc as i64;
        }
        ans
    }
}

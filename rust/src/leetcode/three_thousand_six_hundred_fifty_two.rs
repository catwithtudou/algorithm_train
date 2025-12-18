pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let (mut total, mut sum) = (0, 0);
        for i in 0..k / 2 {
            total += prices[i as usize] as i64 * strategy[i as usize] as i64;
            sum -= prices[i as usize] as i64 * strategy[i as usize] as i64;
        }

        for i in k / 2..k {
            total += prices[i as usize] as i64 * strategy[i as usize] as i64;
            sum += prices[i as usize] as i64 * (1 - strategy[i as usize]) as i64;
        }

        let mut max_num = sum.max(0);

        for i in k..prices.len() as i32 {
            total += prices[i as usize] as i64 * strategy[i as usize] as i64;
            sum += prices[i as usize] as i64 * (1 - strategy[i as usize]) as i64
                - prices[(i - k / 2) as usize] as i64
                + prices[(i - k) as usize] as i64 * strategy[(i - k) as usize] as i64;
            max_num = max_num.max(sum);
        }

        total as i64 + max_num as i64
    }
}

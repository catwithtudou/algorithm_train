pub struct Solution;

impl Solution {
    pub fn minimum_coins(mut prices: Vec<i32>) -> i32 {
        let n = prices.len();
        for i in (1..=(n + 1) / 2 - 1).rev() {
            let min_val = *prices[i..=2*i].into_iter().min().unwrap() as i32;
            prices[i - 1] += min_val;
        }

        prices[0]
    }
}


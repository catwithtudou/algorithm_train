pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];

        for i in 1..=n {
            let min_cost = (i.saturating_sub(3)..i)
                .map(|j| dp[j] + ((i - j) * (i - j)) as i32)
                .min()
                .unwrap();

            dp[i] = min_cost + costs[i - 1];
        }

        dp[n]
    }
}
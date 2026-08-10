pub struct Solution;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let mut dp = vec![false; n + 1];

        for i in 1..=n {
            let mut x = 1;

            while x * x <= i {
                if !dp[i - x * x] {
                    dp[i] = true;
                    break;
                }

                x += 1;
            }
        }

        dp[n]
    }
}
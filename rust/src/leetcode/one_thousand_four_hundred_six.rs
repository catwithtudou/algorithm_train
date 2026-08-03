pub struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();

        // dp[i] 表示从下标 i 开始游戏时，
        // 当前玩家相对另一名玩家最多能领先多少分。
        let mut dp = vec![0; n + 1];

        for i in (0..n).rev() {
            let mut sum = 0;
            let mut best = i32::MIN;

            for j in i..(i + 3).min(n) {
                sum += stone_value[j];
                best = best.max(sum - dp[j + 1]);
            }

            dp[i] = best;
        }

        match dp[0].cmp(&0) {
            std::cmp::Ordering::Greater => "Alice".to_string(),
            std::cmp::Ordering::Less => "Bob".to_string(),
            std::cmp::Ordering::Equal => "Tie".to_string(),
        }
    }
}
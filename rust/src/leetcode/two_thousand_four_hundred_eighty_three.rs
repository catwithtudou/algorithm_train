pub struct Solution;

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut penalty = customers.chars().filter(|&c| c == 'Y').count();
        let mut min_penalty = penalty;
        let mut ans = 0;
        for (i, c) in customers.chars().enumerate() {
            if c == 'N' {
                penalty += 1;
            } else {
                penalty -= 1;
            }
            if penalty < min_penalty {
                min_penalty = penalty;
                ans = i + 1;
            }
        }
        ans as i32
    }
}

pub struct Solution;

use lazy_static::lazy_static;

lazy_static! {
    static ref S: [[i32; 46]; 100001] = {
        let mut s = [[0; 46]; 100001];
        for i in 1..100001 {
            s[i] = s[i - 1];
            s[i][sum_digits(i as i32)] += 1;
        }
        s
    };
}

fn sum_digits(mut x: i32) -> usize {
    let mut sum = 0;
    while x > 0 {
        sum += (x % 10) as usize;
        x /= 10;
    }
    return sum;
}

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let low = low_limit as usize;
        let high = high_limit as usize;
        (1..46)
            .map(|j| S[high][j] - S[low - 1][j])
            .max()
            .unwrap_or(0)
    }
}

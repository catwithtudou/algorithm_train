pub struct Solution;

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        let mut ans = 0;
        piles.sort_unstable();
        let n = piles.len();
        for i in (n / 3..n).step_by(2) {
            ans += piles[i];
        }
        ans
    }
}

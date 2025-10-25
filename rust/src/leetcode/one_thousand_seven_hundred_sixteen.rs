pub struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut week = 1;
        let mut day = 1;
        let mut ans = 0;
        for i in 0..n {
            ans += week + day - 1;
            day += 1;
            if day == 8 {
                day = 1;
                week += 1;
            }
        }
        ans
    }
}

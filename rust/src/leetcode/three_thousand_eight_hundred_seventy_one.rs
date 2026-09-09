pub struct Solution;

impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let mut ans = 0 as i64;
        let mut low = 1000 as i64;
        while low <= n {
            ans += n - low + 1;
            low *= 1000;
        }
        ans
    }
}

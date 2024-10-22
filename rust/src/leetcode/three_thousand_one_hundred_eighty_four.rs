pub struct Solution;

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut ans = 0;
        const H: usize = 24;
        let mut cnt = vec![0; H];
        for t in hours {
            let t = t as usize % H;
            ans += cnt[(H - t) % H];
            cnt[t] += 1;
        }
        ans
    }
}

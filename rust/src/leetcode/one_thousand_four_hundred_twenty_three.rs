pub struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let n = card_points.len();
        let m = n - k as usize;
        let mut s = 0;
        for i in 0..m {
            s += card_points[i];
        }
        let mut total = s;
        let mut min_s = s;
        for i in m..n {
            total += card_points[i];
            s += card_points[i] - card_points[i - m];
            min_s = min_s.min(s);
        }
        total - min_s
    }
}
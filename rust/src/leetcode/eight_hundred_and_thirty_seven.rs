pub struct Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 {
            return 1.0;
        }
        let n = n as usize;
        let k = k as usize;
        let max_pts = max_pts as usize;
        let mut f = vec![0.0; n + 1];
        let mut sum = 0.0;

        for i in (0..=n).rev() {
            f[i] = if i >= k { 1.0 } else { sum / max_pts as f64 };

            sum += f[i];

            if i + max_pts <= n {
                sum -= f[i + max_pts];
            }
        }

        f[0]
    }
}

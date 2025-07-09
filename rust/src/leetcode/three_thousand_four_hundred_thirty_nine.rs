pub struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let k = k as usize;
        let mut free = vec![0; n + 1];
        free[0] = start_time[0];
        for i in 1..n {
            free[i] = start_time[i] - end_time[i - 1];
        }
        free[n] = event_time - end_time[n - 1];

        let mut s = 0;
        let mut ans = 0;
        for i in 0..n + 1 {
            s += free[i];
            if i < k {
                continue;
            }
            ans = ans.max(s);
            s -= free[i - k];
        }
        ans
    }
}

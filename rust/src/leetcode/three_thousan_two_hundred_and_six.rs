pub struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut ans = 0;
        let n = colors.len();
        for i in 0..2 * n {
            if i > 0 && colors[i % n] == colors[(i - 1) % n] {
                cnt = 0;
            }
            cnt += 1;
            if i >= n && cnt >= 3 {
                ans += 1;
            }
        }
        ans
    }
}

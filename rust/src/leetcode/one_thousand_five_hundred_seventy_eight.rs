pub struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut max_t = 0;
        let s = colors.as_bytes();
        for (i, &t) in needed_time.iter().enumerate() {
            ans += t;
            max_t = max_t.max(t);
            if i == s.len() - 1 || s[i] != s[i + 1] {
                ans -= max_t;
                max_t = 0;
            }
        }
        ans
    }
}

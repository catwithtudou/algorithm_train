pub struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut h = 0;
        for d in gain {
            h += d;
            ans = ans.max(h);
        }
        ans
    }
}

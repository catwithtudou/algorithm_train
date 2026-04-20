pub struct Solution;

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len() as i32;
        let c = colors[0];
        if c != colors[n as usize - 1] {
            return n - 1;
        }

        let mut r = n - 2;
        while colors[r as usize] == c {
            r -= 1;
        }

        let mut l = 1;
        while colors[l as usize] == c {
            l += 1;
        }

        r.max(n - l - 1)
    }
}

pub struct Solution;

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let (mut ans, mut x, mut y): (i32, i32, i32) = (0, 0, 0);

        for (i, c) in s.chars().enumerate() {
            if c == 'S' {
                y -= 1;
            } else if c == 'N' {
                y += 1;
            } else if c == 'E' {
                x += 1;
            } else if c == 'W' {
                x -= 1;
            }
            ans = ans.max((x.abs() + y.abs() + 2 * k).min(i as i32 + 1))
        }

        ans
    }
}

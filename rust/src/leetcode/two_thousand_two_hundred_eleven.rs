pub struct Solution;

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let s = directions.as_bytes();
        let n = s.len();

        let mut l = 0;
        while l < n && s[l] == b'L' {
            l += 1;
        }

        let mut r = n;
        while r > l && s[r - 1] == b'R' {
            r -= 1;
        }

        s[l..r].iter().filter(|&&c| c != b'S').count() as i32
    }
}

pub struct Solution;

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let mut vis = vec![vec![false; 26]; 26];
        let s_bytes = s.as_bytes();

        for i in 1..s_bytes.len() {
            let (x, y) = (
                (s_bytes[i - 1] - b'a') as usize,
                (s_bytes[i] - b'a') as usize,
            );
            vis[x][y] = true;
            if vis[y][x] {
                return true;
            }
        }
        false
    }
}

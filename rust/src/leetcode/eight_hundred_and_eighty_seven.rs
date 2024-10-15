pub struct Solution;

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let mut f = vec![vec![0; k as usize + 1]; n as usize + 1];
        let mut i = 1;
        loop {
            for j in 1..=k as usize {
                f[i][j] = f[i - 1][j] + f[i - 1][j - 1] + 1;
            }
            if f[i][k as usize] >= n {
                return i as i32;
            }
            i += 1;
        }
    }
}

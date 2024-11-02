pub struct Solution;

impl Solution {
    pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
        let n = energy_drink_a.len();
        let mut f = vec![vec![0 as i64; 2]; n + 2];
        for i in 0..n {
            f[i + 2][0] = f[i + 1][0].max(f[i][1]) + energy_drink_a[i] as i64;
            f[i + 2][1] = f[i + 1][1].max(f[i][0]) + energy_drink_b[i] as i64;
        }
        return f[n + 1][0].max(f[n + 1][1]);
    }
}

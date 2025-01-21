pub struct Solution;

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut f = vec![vec![0; k + 1]; piles.len() + 1];
        for i in 0..piles.len() {
            for j in 0..k + 1 {
                f[i + 1][j] = f[i][j];
                let mut v = 0;
                for w in 0..j.min(piles[i].len()) {
                    v += piles[i][w];
                    f[i + 1][j] = f[i + 1][j].max(f[i][j - w - 1] + v);
                }
            }
        }
        return f[piles.len()][k];
    }
}

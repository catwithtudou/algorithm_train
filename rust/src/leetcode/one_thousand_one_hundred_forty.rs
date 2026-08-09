pub struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();

        let mut f = vec![vec![0; n + 1]; n];

        let mut suffix_sum = 0;

        for i in (0..n).rev() {
            suffix_sum += piles[i];

            for m in 1..=i / 2 + 1 {
                if i + 2 * m >= n {
                    f[i][m] = suffix_sum;
                } else {
                    let mut min_opponent = i32::MAX;

                    for x in 1..=2 * m {
                        min_opponent =
                            min_opponent.min(f[i + x][m.max(x)]);
                    }

                    f[i][m] = suffix_sum - min_opponent;
                }
            }
        }

        f[0][1]
    }
}
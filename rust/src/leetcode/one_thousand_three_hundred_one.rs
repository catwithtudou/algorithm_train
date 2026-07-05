pub struct Solution;

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;

        let m = board.len();
        let n = board[0].len();

        let mut max_sums = vec![vec![i32::MIN; n + 1]; m + 1];
        let mut ways = vec![vec![0_i64; n + 1]; m + 1];

        max_sums[0][0] = 0;
        ways[0][0] = 1;

        for (i, row) in board.iter().enumerate() {
            for (j, &ch) in row.as_bytes().iter().enumerate() {
                if ch == b'X' {
                    continue;
                }

                let candidates = [
                    (max_sums[i][j], ways[i][j]),         // 左上
                    (max_sums[i][j + 1], ways[i][j + 1]), // 上
                    (max_sums[i + 1][j], ways[i + 1][j]), // 左
                ];

                let best = candidates
                    .iter()
                    .map(|&(score, _)| score)
                    .max()
                    .unwrap();

                let count = candidates
                    .iter()
                    .filter(|&&(score, _)| score == best)
                    .map(|&(_, cnt)| cnt)
                    .sum::<i64>()
                    % MOD;

                max_sums[i + 1][j + 1] = best;
                ways[i + 1][j + 1] = count;

                if ch.is_ascii_digit() {
                    max_sums[i + 1][j + 1] += (ch - b'0') as i32;
                }
            }
        }

        if max_sums[m][n] < 0 {
            vec![0, 0]
        } else {
            vec![max_sums[m][n], ways[m][n] as i32]
        }
    }
}